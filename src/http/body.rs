/// an http body
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    content: Vec<u8>,
}

impl Body {
    /// create an http body from some `content` that is a string
    pub fn new(content: String) -> Body {
        Body {
            content: content.bytes().collect(),
        }
    }

    /// create an http body from raw bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Body {
        Body { content: bytes }
    }

    /// return the content as bytes
    pub fn get_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }

    /// try converting the raw bytes content to a string
    pub fn try_to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.content.clone())
    }
}
