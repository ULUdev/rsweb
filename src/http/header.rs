use super::StatusCode;

/// an http header
pub struct Header {
    status: StatusCode,
    kv_pairs: Vec<String>,
}

impl Header {
    /// create a new http header
    pub fn new(status: StatusCode, kv_pairs: Vec<String>) -> Header {
        Header { status, kv_pairs }
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
}
