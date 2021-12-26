/// types of the SameSite cookie attribute
#[derive(Clone, Debug)]
pub enum SameSiteTypes {
    Strict,
    Lax,
    None,
}

/// attributes a cookie can have
#[derive(Clone, Debug)]
pub enum CookieAttributes {
    Secure,
    HTTPOnly,
    Path(String),
    Domain(String),
    SameSite(SameSiteTypes),
    MaxAge(usize),
    Expires(String),
}


/// a cookie
#[derive(Clone, Debug)]
pub struct Cookie {
    name: String,
    value: String,
    attributes: Option<Vec<CookieAttributes>>,
}

impl Cookie {

    /// create a new cookie
    ///
    /// # Arguments
    /// - `name`: the name of the cookie
    /// - `value`: the value of the cookie
    /// - `attributes`: the cookie attributes (see [`CookieAttributes`] for a list of attributes)
    pub fn new(name: String, value: String, attributes: Option<Vec<CookieAttributes>>) -> Cookie {
        Cookie { name, value, attributes }
    }

    /// create a cookie from a string
    ///
    /// will return `None` if string has invalid format
    pub fn from_string(string: String) -> Option<Cookie> {
        let mut parts = string.split('=');
        let name: String = match parts.next() {
            Some(n) => n.to_string(),
            None => {
                return None;
            }
        };
        let value: String = match parts.next() {
            Some(n) => n.to_string(),
            None => {
                return None;
            }
        };
        Some(Cookie::new(name, value, None))
    }
}
