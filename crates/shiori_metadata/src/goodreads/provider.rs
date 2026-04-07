use crate::{
    goodreads::{book, search},
    provider::{BooksParams, MetadataProvider, MetadataResult},
};
use shiori_api_types::{EncodableBookSearch, EncodableMetadataSearch};

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(params: BooksParams) -> MetadataResult<Vec<EncodableBookSearch>> {
        search::search_books(params).await
    }

    async fn search_id(id: &str) -> MetadataResult<EncodableMetadataSearch> {
        book::search_id(id).await
    }
}
