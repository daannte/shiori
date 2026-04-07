pub use self::errors::MetadataError;
pub use self::goodreads::GoodreadsProvider;
pub use self::provider::MetadataProvider;

mod client;
mod errors;
mod goodreads;
mod provider;
