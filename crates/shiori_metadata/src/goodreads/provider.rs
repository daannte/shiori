use crate::{
    goodreads::{book, search},
    provider::{BooksParams, MetadataProvider, MetadataResult},
};
use futures::{StreamExt, stream};
use shiori_api_types::EncodableMetadataSearch;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(params: BooksParams) -> MetadataResult<Vec<EncodableMetadataSearch>> {
        let ids = search::search_books(params).await?;

        let results = stream::iter(ids)
            .map(|id| async move { Self::search_id(&id).await })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        let books = results.into_iter().filter_map(Result::ok).collect();

        Ok(books)
    }

    async fn search_id(id: &str) -> MetadataResult<EncodableMetadataSearch> {
        book::search_id(id).await
    }
}
