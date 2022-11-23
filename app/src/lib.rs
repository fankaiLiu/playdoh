pub mod apps;
pub mod custom_response;
pub mod error;
pub mod pagination;
pub mod request_query;
pub mod starting;
pub mod utils;
pub mod common;
use custom_response::CustomResponse;
pub use error::Error;
pub mod middleware;
pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type ResponseResult<T, E = Error> = std::result::Result<CustomResponse<T>, E>;

