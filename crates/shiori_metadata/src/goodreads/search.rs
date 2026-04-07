use crate::provider::MetadataProvider;
use crate::{errors::MetadataError, goodreads::parsing::fetch_doc, provider::BooksParams};

pub async fn search_books(params: BooksParams) -> Result<(), MetadataError> {
    let url = format!(
        "{}{}",
        super::GoodreadsProvider::SEARCH_URL,
        params.as_query_string()
    );

    let document = fetch_doc(&url).await?;
    let selector =
        scraper::Selector::parse("table.tableList").map_err(|_| MetadataError::HtmlParse)?;

    let table = document
        .select(&selector)
        .next()
        .ok_or_else(|| MetadataError::MissingTag("books table".to_string()))?;

    let row_selector = scraper::Selector::parse("tr[itemtype='http://schema.org/Book']")
        .map_err(|_| MetadataError::HtmlParse)?;

    let books = table.select(&row_selector);
    for book in books {
        let authors = extract_authors(book);
        println!("{:#?}", authors);
    }

    Ok(())
}

fn extract_authors(book: scraper::ElementRef<'_>) -> Vec<String> {
    let author_selector = match scraper::Selector::parse("a.authorName") {
        Ok(sel) => sel,
        Err(_) => return Vec::new(),
    };

    book.select(&author_selector)
        .filter_map(|e| e.text().next().map(|a| a.trim().to_string()))
        .collect()
}
