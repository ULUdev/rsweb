use crate::http::header::HTTPRequestHeaders;
use crate::http::request::HTTPRequest;
use std::io::Read;

pub struct DBuffer {
    buffer: Vec<u8>,
}

impl DBuffer {
    /// create a new DBuffer
    pub fn new() -> DBuffer {
        DBuffer { buffer: Vec::new() }
    }

    /// create a new DBuffer with `cap` as its capacity
    pub fn with_capacity(cap: usize) -> DBuffer {
        DBuffer {
            buffer: Vec::with_capacity(cap),
        }
    }

    /// read from `r` until it hits a null byte (`\0`)
    ///
    /// # Returns
    /// This function returns a result that if Ok holds the amount of bytes read
    pub fn read_until_zero<T: Read>(&mut self, r: &mut T) -> std::io::Result<usize> {
        let mut tmp_buf = [0u8];
        let mut counter: usize = 0;
        loop {
            match r.read(&mut tmp_buf) {
                Ok(n) => {
                    if n != 1 {
                        break;
                    }
                    if tmp_buf[0] == 0 {
                        break;
                    }
                    self.buffer.push(tmp_buf[0]);
                    counter += 1;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(counter)
    }

    /// read a HTTP request from `r`
    ///
    /// # Returns
    /// This function returns a result that if Ok holds the amount of bytes read
    pub fn read_http_request<T: Read>(&mut self, r: &mut T) -> std::io::Result<usize> {
        let mut tmp_buf = [0u8];
        let mut header_size: usize = 0;
        let mut crlf_counter: u8 = 0;
        let mut cr: bool = false;
        loop {
            match r.read(&mut tmp_buf) {
                Ok(n) => {
                    if n != 1 {
                        break;
                    }
                    if tmp_buf[0] == 0xD {
                        cr = true;
                    } else if cr && tmp_buf[0] == 0xA {
                        crlf_counter += 1;
                    } else {
                        crlf_counter = 0;
                        cr = false;
                    }
                    self.buffer.push(tmp_buf[0]);
                    header_size += 1;
                    if crlf_counter == 2 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        let string = match self.to_string() {
            Ok(n) => n,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "failed to convert bytes to string",
                ));
            }
        };
        let request = match HTTPRequest::from_string(string) {
            Ok(n) => n,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "failed to parse header of HTTP request",
                ));
            }
        };
        let header: Vec<HTTPRequestHeaders> = request.get_header();
        let mut length: usize = 0;
        for head in header {
            if let HTTPRequestHeaders::ContentLength(n) = head {
                length = n;
                break;
            }
        }

        if length == 0 {
            return Ok(header_size);
        } else {
            let mut bytes_read: usize = 0;
            let mut buf = [0u8];
            while bytes_read < length {
                match r.read(&mut buf) {
                    Ok(n) => {
                        if n != 1 {
                            break;
                        }
                        self.buffer.push(buf[0]);
                        bytes_read += 1;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            return Ok(header_size + bytes_read);
        }
    }

    /// try to convert the internal buffer to a string
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.buffer.clone())
    }
}
