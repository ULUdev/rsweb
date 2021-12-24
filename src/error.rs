use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServerError {
    msg: String,
}

impl ServerError {
    pub fn new(msg: &str) -> ServerError {
        ServerError {
            msg: msg.to_string(),
        }
    }
}

impl Error for ServerError {}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
