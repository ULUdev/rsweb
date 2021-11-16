//! # HTTP
//! module containing functions to parse the http protocol
pub mod body;
pub mod header;
pub mod request;
pub mod response;

pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
    MovedPermanently,
}

pub use body::Body;
pub use header::Header;
