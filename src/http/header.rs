// header

use super::status::StatusCode;

pub struct Header {
    status: StatusCode,
    kv_pairs: Vec<String>,
}

impl Header {
    pub fn new(status: StatusCode, kv_pairs: Vec<String>) -> Header {
        Header { status, kv_pairs }
    }
}
