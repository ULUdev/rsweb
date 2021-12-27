use super::cookie::Cookie;
use super::MimeType;
use super::StatusCode;

/// an http response header
#[derive(Clone, Debug)]
#[deprecated(
    since = "0.7.5",
    note = "use `HTTPResponseHeaders` instead. This struct is of no use anymore and will be removed later on"
)]
pub struct HTTPResponseHeader {
    status: StatusCode,
    kv_pairs: Vec<String>,
}

impl HTTPResponseHeader {
    /// create a new http response header
    pub fn new(status: StatusCode) -> HTTPResponseHeader {
        HTTPResponseHeader {
            status,
            kv_pairs: Vec::new(),
        }
    }
    /// parse the headers to a `String`
    pub fn to_string(&self) -> String {
        let stat_string = match self.status {
            StatusCode::Ok => "HTTP/1.1 200 OK\r\n".to_string(),
            StatusCode::NotFound => "HTTP/1.1 404 NOT FOUND\r\n".to_string(),
            StatusCode::InternalServerError => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n".to_string(),
            StatusCode::MovedPermanently => "HTTP/1.1 301 MOVED PERMANENTLY\r\n".to_string(),
        };
        let mut header_string = String::new();
        header_string.push_str(stat_string.as_str());
        let mut header_vals: String = self.kv_pairs.iter().map(|x| format!("{}\r\n", x)).collect();
        if header_vals.len() < 2 {
            header_vals = String::from("\r\n");
        }
        header_vals = header_vals[..header_vals.len() - 2].to_string();
        header_string.push_str(header_vals.as_str());
        header_string
    }

    /// add a new key value pair
    pub fn add_kv_pair(&mut self, k: String, v: String) {
        self.kv_pairs.push(format!("{}:{}", k, v));
    }
}

#[derive(Clone, Debug)]
// TODO: flate2 crate sounds interesting for implementing compression and decompression
// brotli crate for brotli compression algorithm
/// encoding method for content
pub enum ContentEncodingMethod {
    /// gzip compression (Lempel-Ziv coding)
    Gzip,

    /// Lempel-Ziv-Welch compression
    Compress,

    /// deflate compression algorithm (RFC 1951) of the zlib structure (RFC 1950)
    Deflate,

    /// Brotli compression algorithm
    Br,
}

impl ContentEncodingMethod {
    pub fn from_string(string: String) -> Option<ContentEncodingMethod> {
        match string.as_str() {
            "gzip" => Some(ContentEncodingMethod::Gzip),
            "compress" => Some(ContentEncodingMethod::Compress),
            "deflate" => Some(ContentEncodingMethod::Deflate),
            "br" => Some(ContentEncodingMethod::Br),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContentEncodingMethod::Gzip => String::from("gzip"),
            ContentEncodingMethod::Compress => String::from("compress"),
            ContentEncodingMethod::Deflate => String::from("deflate"),
            ContentEncodingMethod::Br => String::from("br"),
        }
    }
}

#[derive(Clone, Debug)]
/// enum representing supported http response headers
pub enum HTTPResponseHeaders {
    ContentType(MimeType),
    ContentLength(usize),
    ContentEncoding(ContentEncodingMethod),
    Server(String),
    SetCookie(Cookie),
    Location(String),
    AccessControlAllowOrigin(String),
}

impl HTTPResponseHeaders {
    /// read the header from a string
    pub fn from_string(string: String) -> Option<HTTPResponseHeaders> {
        let mut parts = string.split(':').map(|x| x.trim_end().trim_start());
        let left: &str = match parts.next() {
            Some(n) => n,
            None => return None,
        };
        let right: &str = match parts.next() {
            Some(n) => n,
            None => return None,
        };
        match left {
            "Content-Type" => {
                if let Some(n) = MimeType::from_string(right.to_string()) {
                    Some(HTTPResponseHeaders::ContentType(n))
                } else {
                    None
                }
            }
            "Content-Length" => {
                if let Err(_) = right.parse::<usize>() {
                    None
                } else {
                    Some(HTTPResponseHeaders::ContentLength(right.parse().unwrap()))
                }
            }
            "Content-Encoding" => {
                if let Some(n) = ContentEncodingMethod::from_string(right.to_string()) {
                    Some(HTTPResponseHeaders::ContentEncoding(n))
                } else {
                    None
                }
            }
            "Server" => Some(HTTPResponseHeaders::Server(right.to_string())),
            "Set-Cookie" => match Cookie::from_string(right.to_string()) {
                Some(n) => Some(HTTPResponseHeaders::SetCookie(n)),
                None => None,
            },
            "Location" => Some(HTTPResponseHeaders::Location(right.to_string())),
            "Access-Control-Allow-Origin" => Some(HTTPResponseHeaders::AccessControlAllowOrigin(
                right.to_string(),
            )),
            _ => None,
        }
    }

    /// convert a HTTPResponseHeaders instance to a string
    pub fn to_string(&self) -> String {
        match self {
            HTTPResponseHeaders::ContentType(n) => format!("Content-Type: {}", n.to_string()),
            HTTPResponseHeaders::ContentLength(n) => format!("Content-Length: {}", n),
            HTTPResponseHeaders::ContentEncoding(n) => {
                format!("Content-Encoding: {}", n.to_string())
            }
            HTTPResponseHeaders::Server(n) => format!("Server: {}", n),
            HTTPResponseHeaders::SetCookie(n) => format!("Set-Cookie: {}", n.to_string()),
            HTTPResponseHeaders::Location(n) => format!("Location: {}", n),
            HTTPResponseHeaders::AccessControlAllowOrigin(n) => {
                format!("Access-Control-Allow-Origin: {}", n)
            }
        }
    }
}

#[derive(Clone, Debug)]
/// enum representing supported http request headers
pub enum HTTPRequestHeaders {
    ContentLength(usize),
    ContentType(MimeType),
    Accept(MimeType),
    UserAgent(String),
    Cookie(Vec<Cookie>),
    Host(String),
}

impl HTTPRequestHeaders {
    pub fn from_string(string: String) -> Option<HTTPRequestHeaders> {
        let mut parts = string.split(':');
        let key = match parts.next() {
            Some(n) => n,
            None => {
                return None;
            }
        };
        let value = match parts.next() {
            Some(n) => n,
            None => {
                return None;
            }
        };
        let value = value.trim_end();
        let value = value.trim_start();
        match key {
            "Content-Length" => {
                if let Err(_) = value.parse::<usize>() {
                    return None;
                }
                Some(HTTPRequestHeaders::ContentLength(value.parse().unwrap()))
            }
            "Content-Type" => {
                if let Some(m) = MimeType::from_string(value.to_string()) {
                    return Some(HTTPRequestHeaders::ContentType(m));
                } else {
                    return None;
                }
            }
            "Accept" => {
                if let Some(m) = MimeType::from_string(value.to_string()) {
                    return Some(HTTPRequestHeaders::ContentType(m));
                } else {
                    return None;
                }
            }
            "User-Agent" => Some(HTTPRequestHeaders::UserAgent(value.to_string())),
            "Cookie" => {
                let parts: Vec<String> = value.split("; ").map(|x| x.to_string()).collect();
                let mut cookies: Vec<Cookie> = Vec::new();
                for part in parts {
                    if let Some(cookie) = Cookie::from_string(part) {
                        cookies.push(cookie);
                    }
                }
                Some(HTTPRequestHeaders::Cookie(cookies))
            }
            "Host" => Some(HTTPRequestHeaders::Host(value.to_string())),
            _ => None,
        }
    }
}

/// create a vector of HTTPRequestHeaders from a string
pub fn req_headers_from_string(string: String) -> Vec<HTTPRequestHeaders> {
    let mut out: Vec<HTTPRequestHeaders> = Vec::new();
    for line in string.lines() {
        if let Some(n) = HTTPRequestHeaders::from_string(line.to_string()) {
            out.push(n);
        }
    }
    out
}
