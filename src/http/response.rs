use super::header::HTTPResponseHeaders;
use super::Body;
use super::StatusCode;

pub struct HTTPResponse {
    status: StatusCode,
    header: Vec<HTTPResponseHeaders>,
    body: Body,
}

impl HTTPResponse {
    pub fn new(status: StatusCode, header: Vec<HTTPResponseHeaders>, body: Body) -> HTTPResponse {
        HTTPResponse {
            status,
            header,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut header = String::new();
        for i in &self.header {
            header.push_str(format!("{}\r\n", i.to_string()).as_str());
        }
        format!("{}\r\n{}\r\n{}", self.status.to_string(), header, self.body)
    }
}
