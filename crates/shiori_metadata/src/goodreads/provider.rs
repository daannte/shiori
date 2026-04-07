use crate::{
    errors::MetadataError,
    goodreads::{book, search},
    provider::{BooksParams, MetadataProvider},
};
use shiori_api_types::EncodableMetadataSearch;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(params: BooksParams) -> Result<(), MetadataError> {
        search::search_books(params).await
    }

    async fn search_id(id: &str) -> Result<EncodableMetadataSearch, MetadataError> {
        book::search_id(id).await
    }
}
