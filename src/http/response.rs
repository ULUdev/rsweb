use super::header::HTTPResponseHeaders;
use super::Body;
use super::StatusCode;

pub struct HTTPResponse {
    status: StatusCode,
    header: Vec<HTTPResponseHeaders>,
    body: Body,
}

impl HTTPResponse {
    /// create a new HTTPResponse
    pub fn new(status: StatusCode, header: Vec<HTTPResponseHeaders>, body: Body) -> HTTPResponse {
        HTTPResponse {
            status,
            header,
            body,
        }
    }

    /// try to convert a response to a string
    pub fn try_to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        let mut header = String::new();
        for i in &self.header {
            header.push_str(format!("{}\r\n", i.to_string()).as_str());
        }
        match self.body.try_to_string() {
            Ok(n) => Ok(format!(
                "{}\r\n{}\r\n{}",
                self.status.to_string(),
                header,
                n
            )),
            Err(e) => Err(e),
        }
    }

    /// convert a response to a bytes vector
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for i in self.status.to_string().bytes() {
            bytes.push(i);
        }
        bytes.push(0xD); // carriage return
        bytes.push(0xA); // linefeed
        for i in &self.header {
            for j in i.to_string().bytes() {
                bytes.push(j);
            }
            bytes.push(0xD); // carriage return
            bytes.push(0xA); // linefeed
        }
        bytes.push(0xD); // carriage return
        bytes.push(0xA); // linefeed
        for i in self.body.get_bytes() {
            bytes.push(i);
        }

        bytes
    }
}
