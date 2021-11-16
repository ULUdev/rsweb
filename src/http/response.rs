use super::Body;
use super::Header;

pub struct HTTPResponse {
    header: Header,
    body: Body,
}

impl HTTPResponse {
    pub fn new(header: Header, body: Body) -> HTTPResponse {
        HTTPResponse { header, body }
    }

    pub fn to_string(&self) -> String {
        format!("{}\r\n\r\n{}", self.header.to_string(), self.body)
    }
}
