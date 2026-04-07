use crate::{
    errors::MetadataError,
    goodreads::{book, search},
    provider::MetadataProvider,
};
use shiori_api_types::EncodableMetadataSearch;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(search_term: &str) -> Result<(), MetadataError> {
        search::search_books(search_term).await
    }

    async fn search_id(id: &str) -> Result<EncodableMetadataSearch, MetadataError> {
        book::search_id(id).await
    }
}
