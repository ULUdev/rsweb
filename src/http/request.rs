#[derive(Debug)]
pub struct HTTPRequestParsingError;

impl std::error::Error for HTTPRequestParsingError {}

impl std::fmt::Display for HTTPRequestParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "error parsing HTTP request")
    }
}

/// An HTTP method
#[derive(Clone, Debug)]
pub enum HTTPMethod {
    Get,
    Post,
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let string = match self {
            HTTPMethod::Get => String::from("GET"),
            HTTPMethod::Post => String::from("POST"),
        };
        write!(f, "{}", string)
    }
}

/// An HTTP request
#[derive(Clone, Debug)]
pub struct HTTPRequest {
    method: HTTPMethod,
    path: String,
}

impl HTTPRequest {
    /// construct a new HTTP request
    pub fn new(method: HTTPMethod, path: String) -> HTTPRequest {
        HTTPRequest { method, path }
    }

    pub fn from_string(req_string: String) -> Result<HTTPRequest, HTTPRequestParsingError> {
        let line1: String = req_string.lines().next().unwrap().to_string();
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
            _ => {
                return Err(HTTPRequestParsingError);
            }
        };
        Ok(HTTPRequest::new(method, path.to_string()))
    }

    /// get the path
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// get the method
    pub fn get_method(&self) -> HTTPMethod {
        self.method.clone()
    }
}
