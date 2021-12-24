//! # HTTP
//! module containing functions to parse the http protocol
pub mod body;
pub mod header;
pub mod request;
pub mod response;

/// enum for supported http status codes
pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
    MovedPermanently,
}

/// enum for supported mime types
#[derive(Clone)]
pub enum MimeType {
    Html,
    Javascript,
    Css,
    Jpeg,
    Png,
    Pdf,
    Plaintext,
    Other(String),
}

impl MimeType {
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Html => String::from("text/html"),
            MimeType::Javascript => String::from("text/javascript"),
            MimeType::Css => String::from("text/css"),
            MimeType::Jpeg => String::from("image/jpeg"),
            MimeType::Png => String::from("image/png"),
            MimeType::Pdf => String::from("application/pdf"),
            MimeType::Plaintext => String::from("text/plain"),
            MimeType::Other(n) => n.to_string(),
        }
    }
}

pub use body::Body;
pub use header::Header;
