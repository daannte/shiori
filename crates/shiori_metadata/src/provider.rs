use shiori_api_types::{EncodableBookSearch, EncodableMetadataSearch};
use url::form_urlencoded;

use crate::errors::MetadataError;

pub type MetadataResult<T> = Result<T, MetadataError>;

#[allow(async_fn_in_trait)]
pub trait MetadataProvider {
    const BOOK_URL: &str;
    const SEARCH_URL: &str;

    async fn search_books(search_term: BooksParams) -> MetadataResult<Vec<EncodableBookSearch>>;
    async fn search_id(id: &str) -> MetadataResult<EncodableMetadataSearch>;
}

pub struct BooksParams {
    pub author: String,
    pub title: String,
}

impl BooksParams {
    pub fn as_query_string(&self) -> String {
        let raw = format!("{} {}", self.author, self.title);

        form_urlencoded::byte_serialize(raw.as_bytes()).collect()
    }
}
