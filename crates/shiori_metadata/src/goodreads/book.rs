use chrono::DateTime;
use serde_json::Value;
use shiori_api_types::EncodableMetadataSearch;

use crate::provider::{MetadataProvider, MetadataResult};
use crate::{MetadataError, goodreads::parsing::fetch_doc};

pub async fn search_id(book: &str) -> MetadataResult<EncodableMetadataSearch> {
    let url = format!("{}{}", super::GoodreadsProvider::BOOK_URL, book);

    let document = match fetch_doc(&url).await {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!(%book, "Failed to fetch document: {:?}", e);
            return Err(MetadataError::Network(e));
        }
    };
    let selector = scraper::Selector::parse("script#__NEXT_DATA__").unwrap();

    let script = match document.select(&selector).next() {
        Some(script) => script,
        None => {
            let error_msg = "__NEXT_DATA__ script tag not found";
            tracing::error!(%book, "{}", error_msg);
            return Err(MetadataError::MissingTag(error_msg.to_string()));
        }
    };

    let next_data: Value = match serde_json::from_str(&script.inner_html()) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!(%book, "Failed to parse JSON data: {}", e);
            return Err(MetadataError::JsonParse(e));
        }
    };

    let apollo_state = &next_data["props"]["pageProps"]["apolloState"];

    let book_info = match book_info(apollo_state) {
        Some(info) => info,
        None => {
            tracing::error!(%book, "Missing book info");
            return Err(MetadataError::MissingBookInfo);
        }
    };

    let title = book_info
        .get("title")
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| {
            let error_msg = "Missing title".to_string();
            tracing::error!(%book, "{}", error_msg);
            MetadataError::Other(error_msg)
        })?;

    let provider_id = extract_id(book_info).ok_or_else(|| {
        let error_msg = "Missing provider_id".to_string();
        tracing::error!(%book, "{}", error_msg);
        MetadataError::Other(error_msg)
    })?;

    let mut metadata = EncodableMetadataSearch {
        title,
        authors: extract_author_names(apollo_state, book_info),
        cover_url: book_info
            .get("imageUrl")
            .and_then(Value::as_str)
            .map(String::from),
        description: book_info
            .get("description")
            .and_then(Value::as_str)
            .map(String::from),
        genres: extract_genres(book_info),
        provider_id,
        ..Default::default()
    };

    if let Some(details) = book_info.get("details") {
        metadata.publisher = details
            .get("publisher")
            .and_then(Value::as_str)
            .map(String::from);

        metadata.isbn = details
            .get("isbn")
            .and_then(Value::as_str)
            .map(String::from);

        metadata.language = details
            .get("language")
            .and_then(|l| l.get("name"))
            .and_then(Value::as_str)
            .map(String::from);

        metadata.published_at = details
            .get("publicationTime")
            .and_then(|time| time.as_i64())
            .and_then(DateTime::from_timestamp_millis)
            .map(|datetime| datetime.date_naive())
    }

    tracing::info!(%book, "Successfully fetched metadata");

    Ok(metadata)
}

/// Extract the first key starting with "book:" and return its value
fn book_info(apollo_state: &Value) -> Option<&Value> {
    apollo_state
        .as_object()?
        .iter()
        .find(|(k, _)| k.to_lowercase().starts_with("book:"))
        .map(|(_, v)| v)
}

/// Extract the goodreads id. Every book on goodreads should
/// have a legacy id.
fn extract_id(book_info: &Value) -> Option<u32> {
    book_info
        .get("legacyId")
        .and_then(|v| v.as_u64().and_then(|n| u32::try_from(n).ok()))
        .or_else(|| {
            book_info
                .get("legacyId")
                .and_then(Value::as_str)
                .and_then(|s| s.parse::<u32>().ok())
        })
}

/// Return a list of genres
fn extract_genres(book_info: &Value) -> Vec<String> {
    book_info
        .get("bookGenres")
        .and_then(|g| g.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|g| {
                    g.get("genre")
                        .and_then(|g| g.get("name"))
                        .and_then(Value::as_str)
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Given a reference key, return the author's name if it exists
fn get_author_name<'a>(apollo_state: &'a Value, r: Option<&'a str>) -> Option<&'a str> {
    let key = r?;
    apollo_state.as_object()?.get(key)?.get("name")?.as_str()
}

/// Return a list of all author names
fn extract_author_names(apollo_state: &Value, book_info: &Value) -> Vec<String> {
    let mut authors = Vec::new();

    // Primary author
    let primary_ref = book_info
        .get("primaryContributorEdge")
        .and_then(|e| e.get("node"))
        .and_then(|n| n.get("__ref"))
        .and_then(Value::as_str);

    if let Some(name) = get_author_name(apollo_state, primary_ref) {
        authors.push(name);
    }

    // Secondary authors
    if let Some(edges) = book_info
        .get("secondaryContributorEdges")
        .and_then(Value::as_array)
    {
        for edge in edges {
            let is_author = edge
                .get("role")
                .and_then(Value::as_str)
                .map(|r| r.eq_ignore_ascii_case("author"))
                .unwrap_or(false);

            if !is_author {
                continue;
            }

            let node_ref = edge
                .get("node")
                .and_then(|n| n.get("__ref"))
                .and_then(Value::as_str);
            if let Some(name) = get_author_name(apollo_state, node_ref) {
                authors.push(name);
            }
        }
    }

    authors.iter().map(|s| s.to_string()).collect()
}
