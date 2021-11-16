pub struct Body {
    content: String,
}

impl Body {
    pub fn new(content: String) -> Body {
        Body { content }
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.content)
    }
}
