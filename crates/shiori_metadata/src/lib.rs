pub use self::errors::MetadataError;
pub use self::goodreads::GoodreadsProvider;

mod client;
mod errors;
mod goodreads;
pub mod provider;
