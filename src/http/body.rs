/// an http body
pub struct Body {
    content: String,
}

impl Body {
    /// create an http body from some `content`
    pub fn new(content: String) -> Body {
        Body { content }
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.content)
    }
}
