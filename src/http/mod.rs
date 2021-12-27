//! # HTTP
//! module containing functions to parse the http protocol
pub mod body;
pub mod cookie;
pub mod header;
pub mod request;
pub mod response;

/// enum for supported http status codes
#[derive(Clone, Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
    MovedPermanently,
}

impl StatusCode {
    /// convert an HTTP status code to a string
    pub fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => String::from("HTTP/1.1 200 OK"),
            StatusCode::NotFound => String::from("HTTP/1.1 404 NOT FOUND"),
            StatusCode::InternalServerError => String::from("HTTP/1.1 500 INTERNAL SERVER ERROR"),
            StatusCode::MovedPermanently => String::from("HTTP/1.1 301 MOVED PERMANENTLY"),
        }
    }
}

/// enum for supported mime types
#[derive(Clone, Debug)]
pub enum MimeType {
    Html,
    Javascript,
    Css,
    Jpeg,
    Png,
    Pdf,
    Plaintext,
    MultipartFormData,
    WWWFormUrlencoded,
    Other(String),
    Wildcard(String, String),
}

impl MimeType {
    /// make a MimeType from a string and return `None` if it is a non recognizable MimeType
    pub fn from_string(string: String) -> Option<MimeType> {
        match string.as_str() {
            "text/html" => Some(MimeType::Html),
            "text/javascript" => Some(MimeType::Javascript),
            "text/css" => Some(MimeType::Css),
            "image/jpeg" => Some(MimeType::Jpeg),
            "image/png" => Some(MimeType::Png),
            "application/pdf" => Some(MimeType::Pdf),
            "text/pdf" => Some(MimeType::Plaintext),
            "multipart/form-data" => Some(MimeType::MultipartFormData),
            "application/x-www-form-urlencoded" => Some(MimeType::WWWFormUrlencoded),
            n => {
                if n.contains("*") {
                    let mut parts = n.split('/');
                    let l = parts.next();
                    let r = parts.next();
                    if l == None || r == None {
                        None
                    } else {
                        let l = l.unwrap();
                        let r = r.unwrap();
                        Some(MimeType::Wildcard(l.to_string(), r.to_string()))
                    }
                } else if n.contains('/') {
                    Some(MimeType::Other(n.to_string()))
                } else {
                    None
                }
            }
        }
    }

    /// stringify a mime type
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Html => String::from("text/html"),
            MimeType::Javascript => String::from("text/javascript"),
            MimeType::Css => String::from("text/css"),
            MimeType::Jpeg => String::from("image/jpeg"),
            MimeType::Png => String::from("image/png"),
            MimeType::Pdf => String::from("application/pdf"),
            MimeType::Plaintext => String::from("text/plain"),
            MimeType::MultipartFormData => String::from("multipart/form-data"),
            MimeType::WWWFormUrlencoded => String::from("application/x-www-form-urlencoded"),
            MimeType::Other(n) => n.to_string(),
            MimeType::Wildcard(l, r) => format!("{}/{}", l, r),
        }
    }
}

pub use body::Body;
pub use header::HTTPResponseHeader;
#[deprecated(since = "0.6.5", note = "use `HTTPResponseHeader` type instead")]
pub type Header = HTTPResponseHeader;
