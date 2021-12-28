use super::header::HTTPRequestHeaders;

#[derive(Debug)]
pub struct HTTPRequestParsingError;

impl std::error::Error for HTTPRequestParsingError {}

impl std::fmt::Display for HTTPRequestParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "error parsing HTTP request")
    }
}

/// An HTTP method
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HTTPMethod {
    Get,
    Post,
    Head,
    Delete,
    Put,
    Connect,
    Options,
    Trace,
    Patch,
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let string = match self {
            HTTPMethod::Get => String::from("GET"),
            HTTPMethod::Post => String::from("POST"),
            HTTPMethod::Head => String::from("HEAD"),
            HTTPMethod::Delete => String::from("DELETE"),
            HTTPMethod::Put => String::from("PUT"),
            HTTPMethod::Connect => String::from("CONNECT"),
            HTTPMethod::Options => String::from("OPTIONS"),
            HTTPMethod::Trace => String::from("TRACE"),
            HTTPMethod::Patch => String::from("PATCH"),
        };
        write!(f, "{}", string)
    }
}

/// An HTTP request
#[derive(Clone, Debug)]
pub struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    header: Vec<HTTPRequestHeaders>,
    body: Option<String>,
}

impl HTTPRequest {
    /// construct a new HTTP request
    pub fn new(
        method: HTTPMethod,
        path: String,
        header: Vec<HTTPRequestHeaders>,
        body: Option<String>,
    ) -> HTTPRequest {
        // TODO: proper reading of header from arguments
        HTTPRequest {
            method,
            path,
            body,
            header,
        }
    }

    pub fn from_string(req_string: String) -> Result<HTTPRequest, HTTPRequestParsingError> {
        let mut lines = req_string.lines();
        let line1: String = match lines.next() {
            Some(n) => n.to_string(),
            None => {
                return Err(HTTPRequestParsingError);
            }
        };
        let mut line1_iter = line1.split_whitespace();
        let method: &str = match line1_iter.next() {
            Some(n) => n,
            None => {
                return Err(HTTPRequestParsingError);
            }
        };
        let path: &str = match line1_iter.next() {
            Some(n) => n,
            None => {
                return Err(HTTPRequestParsingError);
            }
        };

        let method: HTTPMethod = match method {
            "GET" => HTTPMethod::Get,
            "POST" => HTTPMethod::Post,
            "HEAD" => HTTPMethod::Head,
            "DELETE" => HTTPMethod::Delete,
            "PUT" => HTTPMethod::Put,
            "CONNECT" => HTTPMethod::Connect,
            "OPTIONS" => HTTPMethod::Options,
            "TRACE" => HTTPMethod::Trace,
            "PATCH" => HTTPMethod::Patch,
            _ => {
                return Err(HTTPRequestParsingError);
            }
        };
        let mut out_headers: Vec<HTTPRequestHeaders> = Vec::new();
        for line in req_string.lines().skip(1) {
            if let Some(n) = HTTPRequestHeaders::from_string(line.to_string()) {
                out_headers.push(n);
            }
        }
        let body: Option<String> = match path.split("\r\n\r\n").nth(1) {
            Some(n) => Some(n.to_string()),
            None => None,
        };
        Ok(HTTPRequest::new(
            method,
            path.to_string(),
            out_headers,
            body,
        ))
    }

    /// get the path
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// get the method
    pub fn get_method(&self) -> HTTPMethod {
        self.method.clone()
    }

    /// get the header
    pub fn get_header(&self) -> Vec<HTTPRequestHeaders> {
        self.header.clone()
    }
}
