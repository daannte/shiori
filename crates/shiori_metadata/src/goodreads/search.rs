use url::form_urlencoded;

use crate::{MetadataProvider, errors::MetadataError, goodreads::parsing::fetch_doc};

pub async fn search_books(search_term: &str) -> Result<(), MetadataError> {
    let encoded: String = form_urlencoded::byte_serialize(search_term.as_bytes()).collect();
    let url = format!("{}{}", super::GoodreadsProvider::BOOK_URL, encoded);

    let document = fetch_doc(&url).await?;
    let selector =
        scraper::Selector::parse("table.tableList").map_err(|_| MetadataError::HtmlParse)?;

    let script = document
        .select(&selector)
        .next()
        .ok_or_else(|| MetadataError::MissingTag("books table".to_string()))?;

    todo!()
}
