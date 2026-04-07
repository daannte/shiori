use shiori_api_types::EncodableBookSearch;

use crate::provider::{MetadataProvider, MetadataResult};
use crate::{goodreads::parsing::fetch_doc, provider::BooksParams};

pub async fn search_books(params: BooksParams) -> MetadataResult<Vec<EncodableBookSearch>> {
    let url = format!(
        "{}{}",
        super::GoodreadsProvider::SEARCH_URL,
        params.as_query_string()
    );

    let document = fetch_doc(&url).await?;
    let selector = scraper::Selector::parse("table.tableList").unwrap();

    let table = match document.select(&selector).next() {
        Some(table) => table,
        None => return Ok(Vec::new()),
    };

    let row_selector = scraper::Selector::parse("tr[itemtype='http://schema.org/Book']").unwrap();

    let books = table.select(&row_selector);

    let mut res = Vec::new();

    for book in books {
        // TODO: some sort of fuzzy/scoring system
        // to pick good books with authors and title
        let title = extract_title(book);
        let authors = extract_authors(book);
        res.push(EncodableBookSearch {
            title,
            authors,
            cover_url: extract_cover_url(book),
        });
    }

    Ok(res)
}

fn extract_title(book: scraper::ElementRef<'_>) -> String {
    let title_selector = scraper::Selector::parse("a[title]").unwrap();

    book.select(&title_selector)
        .next()
        .and_then(|e| e.value().attr("title"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn extract_authors(book: scraper::ElementRef<'_>) -> Vec<String> {
    let author_selector = scraper::Selector::parse("a.authorName").unwrap();

    book.select(&author_selector)
        .filter_map(|e| e.text().next().map(|a| a.trim().to_string()))
        .collect()
}

fn extract_cover_url(book: scraper::ElementRef<'_>) -> Option<String> {
    let img_selector = scraper::Selector::parse("img").unwrap();

    book.select(&img_selector)
        .next()?
        .value()
        .attr("src")
        .map(String::from)
}
