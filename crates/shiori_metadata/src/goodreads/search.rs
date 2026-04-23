use scraper::{ElementRef, Selector};

use crate::MetadataError;
use crate::goodreads::parsing::fetch_doc;
use crate::provider::{MetadataProvider, MetadataResult};

pub async fn books(q: String) -> MetadataResult<Vec<String>> {
    let url = format!("{}{}", super::GoodreadsProvider::SEARCH_URL, q);

    tracing::debug!("Fetching book data from URL: {}", url);

    let document = match fetch_doc(&url).await {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!("Error fetching document for books: {}", e);
            return Err(MetadataError::Network(e));
        }
    };
    let selector = Selector::parse("table.tableList").unwrap();

    let table = match document.select(&selector).next() {
        Some(table) => table,
        None => {
            tracing::info!("No table found in document");
            return Ok(Vec::new());
        }
    };

    let row_selector = Selector::parse("tr[itemtype='http://schema.org/Book']").unwrap();

    let books = table.select(&row_selector);

    let mut res = Vec::new();

    // TODO: some sort of fuzzy/scoring system
    // to pick good books with authors and title
    for book in books {
        let id = extract_id(book);

        if id.is_none() {
            tracing::warn!("Failed to extract ID from a book, skipping...");
            continue;
        }

        res.push(id.unwrap());
    }

    Ok(res)
}

fn extract_id(book: ElementRef<'_>) -> Option<String> {
    let id_selector = Selector::parse("div[id]").unwrap();

    book.select(&id_selector)
        .next()
        .and_then(|div| div.value().attr("id"))
        .map(|s| s.trim().to_string())
}

fn _extract_title(book: ElementRef<'_>) -> String {
    let title_selector = Selector::parse("a[title]").unwrap();

    book.select(&title_selector)
        .next()
        .and_then(|e| e.value().attr("title"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| {
            tracing::warn!("Failed to extract title from book");
            String::default()
        })
}

fn _extract_authors(book: ElementRef<'_>) -> Vec<String> {
    let author_selector = Selector::parse("a.authorName").unwrap();

    book.select(&author_selector)
        .filter_map(|e| e.text().next().map(|a| a.trim().to_string()))
        .collect()
}

fn _extract_cover_url(book: ElementRef<'_>) -> Option<String> {
    let img_selector = Selector::parse("img").unwrap();

    book.select(&img_selector)
        .next()?
        .value()
        .attr("src")
        .map(String::from)
}
