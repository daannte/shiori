use shiori_api_types::EncodableMetadataSearch;

pub type MetadataResult<T> = Result<T, crate::MetadataError>;

#[allow(async_fn_in_trait)]
pub trait MetadataProvider {
    const BOOK_URL: &str;
    const SEARCH_URL: &str;

    async fn search_books(q: String) -> MetadataResult<Vec<EncodableMetadataSearch>>;
    async fn fetch_book(id: String) -> MetadataResult<EncodableMetadataSearch>;
}
