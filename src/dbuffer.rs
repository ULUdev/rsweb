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
                    } else if tmp_buf[0] == 0 {
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

    /// read from `r` until ia HTTP request end is met (`\r\n\r\n` or `0xD 0xA 0xD 0xA`)
    ///
    /// # Returns
    /// This function returns a result that if Ok holds the amount of bytes read
    pub fn read_until_req_end<T: Read>(&mut self, r: &mut T) -> std::io::Result<usize> {
        let mut tmp_buf = [0u8];
        let mut counter: usize = 0;
        let mut crlf_counter: u8 = 0;
        loop {
            match r.read(&mut tmp_buf) {
                Ok(n) => {
                    if n != 1 {
                        break;
                        // TODO: currently order doesn't matter. It might be 0xA 0xA 0xD 0xD and still
                        // end reading
                    }
                    if tmp_buf[0] == 13 || tmp_buf[0] == 10 {
                        crlf_counter += 1;
                    } else {
                        crlf_counter = 0;
                    }
                    self.buffer.push(tmp_buf[0]);
                    counter += 1;
                    if crlf_counter == 4 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(counter)
    }

    /// try to convert the internal buffer to a string
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.buffer.clone())
    }
}
