use shiori_api_types::EncodableMetadataSearch;

use crate::errors::MetadataError;

#[allow(async_fn_in_trait)]
pub trait MetadataProvider {
    const BOOK_URL: &str;
    const SEARCH_URL: &str;

    async fn search_books(search_term: &str) -> Result<(), MetadataError>;
    async fn search_id(id: &str) -> Result<EncodableMetadataSearch, MetadataError>;
}
