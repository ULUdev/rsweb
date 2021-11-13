//! # HTTP
//! module containing functions to parse the http protocol
pub mod header;
pub mod body;
pub mod request;
pub mod response;

pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
}

pub use header::Header;
pub use body::Body;
