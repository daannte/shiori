use crate::{errors::MetadataError, provider::MetadataProvider};
use chrono::DateTime;
use serde_json::Value;
use shiori_api_types::EncodableMetadataSearch;

pub struct GoodreadsProvider;

// TODO: Def refactor this, i dont like how it looks rn lol
//
// Would prob be a good idea to make my own types for this.
impl MetadataProvider for GoodreadsProvider {
    const URL: &str = "https://www.goodreads.com/book/show/";

    async fn search(id: &str) -> Result<EncodableMetadataSearch, MetadataError> {
        let body = reqwest::get(format!("{}{}", Self::URL, id))
            .await?
            .error_for_status()?
            .text()
            .await?;

        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("script#__NEXT_DATA__")
            .map_err(|_| MetadataError::HtmlParse)?;

        let script = document
            .select(&selector)
            .next()
            .ok_or_else(|| MetadataError::MissingScriptTag)?;

        let next_data: Value = serde_json::from_str(&script.inner_html())?;

        let apollo_state = &next_data["props"]["pageProps"]["apolloState"];

        let book_info =
            Self::book_info(apollo_state).ok_or_else(|| MetadataError::MissingBookInfo)?;

        let mut metadata = EncodableMetadataSearch {
            title: book_info
                .get("title")
                .and_then(Value::as_str)
                .map(String::from)
                .unwrap_or_default(),
            authors: Self::extract_author_names(apollo_state, book_info),
            cover_url: book_info
                .get("imageUrl")
                .and_then(Value::as_str)
                .map(String::from),
            description: book_info
                .get("description")
                .and_then(Value::as_str)
                .map(String::from),
            genres: Self::extract_genres(book_info),
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

        Ok(metadata)
    }
}

// TODO: maybe extract people like translators, illustrator, etc.
impl GoodreadsProvider {
    /// Extract the first key starting with "book:" and return its value
    fn book_info(apollo_state: &Value) -> Option<&Value> {
        apollo_state
            .as_object()?
            .iter()
            .find(|(k, _)| k.to_lowercase().starts_with("book:"))
            .map(|(_, v)| v)
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

        if let Some(name) = Self::get_author_name(apollo_state, primary_ref) {
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
                if let Some(name) = Self::get_author_name(apollo_state, node_ref) {
                    authors.push(name);
                }
            }
        }

        authors.iter().map(|s| s.to_string()).collect()
    }
}
