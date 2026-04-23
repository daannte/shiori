use crate::{
    MetadataError,
    client::HTTP_CLIENT,
    goodreads::{book, search},
    is_isbn, is_url_encoded,
    provider::{MetadataProvider, MetadataResult},
};
use futures::{StreamExt, stream};
use shiori_api_types::EncodableMetadataSearch;
use url::form_urlencoded;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(q: String) -> MetadataResult<Vec<EncodableMetadataSearch>> {
        let q = q.trim().to_string();

        if q.is_empty() {
            return Ok(vec![]);
        }

        if is_isbn(&q) {
            let id = Self::isbn_to_id(&q).await?;
            let book = Self::fetch_book(id).await?;
            return Ok(vec![book]);
        }

        let q = if is_url_encoded(&q) {
            q
        } else {
            form_urlencoded::byte_serialize(q.as_bytes()).collect()
        };

        let ids = search::books(q).await?;

        let results = stream::iter(ids)
            .map(|id| async move { Self::fetch_book(id).await })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        let books = results
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(books)
    }

    async fn fetch_book(id: String) -> MetadataResult<EncodableMetadataSearch> {
        book::fetch(id).await
    }
}

impl GoodreadsProvider {
    async fn isbn_to_id(isbn: &str) -> MetadataResult<String> {
        let url = format!("{}{}", Self::SEARCH_URL, isbn);

        let res = HTTP_CLIENT.get(&url).send().await?;

        let path = res.url().path();

        let id = path
            .split('/')
            .nth(3)
            .and_then(|s| s.split('-').next())
            .ok_or(MetadataError::Other("Not Found".to_string()))?;

        Ok(id.to_string())
    }
}
