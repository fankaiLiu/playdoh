pub mod apps;
pub mod custom_response;
pub mod error;
pub mod pagination;
pub mod request_query;
pub mod starting;
pub mod utils;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
