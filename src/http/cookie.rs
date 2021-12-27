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

impl CookieAttributes {
    pub fn to_string(&self) -> String {
        match self {
            CookieAttributes::Secure => String::from("Secure"),
            CookieAttributes::HTTPOnly => String::from("HttpOnly"),
            CookieAttributes::Path(n) => format!("Path={}", n),
            CookieAttributes::Domain(n) => format!("Domain={}", n),
            CookieAttributes::SameSite(n) => match n {
                SameSiteTypes::Strict => String::from("SameSite=Strict"),
                SameSiteTypes::Lax => String::from("SameSite=Lax"),
                SameSiteTypes::None => String::from("SameSite=None"),
            },
            CookieAttributes::MaxAge(n) => format!("Max-Age={}", n),
            CookieAttributes::Expires(n) => format!("Expires={}", n),
        }
    }
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
        Cookie {
            name,
            value,
            attributes,
        }
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
        let mut other_parts = string.split("; ");
        let left = match other_parts.next() {
            Some(n) => n,
            None => return None,
        };
        let value = match left.split('=').nth(1) {
            Some(n) => n,
            None => return None,
        };
        let mut attributes: Vec<CookieAttributes> = Vec::new();
        for i in string.split("; ").skip(1) {
            if i.contains('=') {
                let mut parts = i.split('=');
                let left = match parts.next() {
                    Some(n) => n,
                    None => break,
                };
                let right = match parts.next() {
                    Some(n) => n,
                    None => break,
                };
                match left {
                    "Expires" => attributes.push(CookieAttributes::Expires(right.to_string())),
                    "Max-Age" => {
                        if let Ok(n) = right.parse::<usize>() {
                            attributes.push(CookieAttributes::MaxAge(n));
                        }
                    }
                    "Domain" => attributes.push(CookieAttributes::Domain(right.to_string())),
                    "Path" => attributes.push(CookieAttributes::Path(right.to_string())),
                    "SameSite" => match right {
                        "Lax" => attributes.push(CookieAttributes::SameSite(SameSiteTypes::Lax)),
                        "Strict" => {
                            attributes.push(CookieAttributes::SameSite(SameSiteTypes::Strict))
                        }
                        "None" => attributes.push(CookieAttributes::SameSite(SameSiteTypes::None)),
                        _ => (),
                    },
                    _ => (),
                }
            } else {
                match i {
                    "Secure" => attributes.push(CookieAttributes::Secure),
                    "HttpOnly" => attributes.push(CookieAttributes::HTTPOnly),
                    _ => (),
                }
            }
        }
        let out_attributes: Option<Vec<CookieAttributes>> = match attributes.len() {
            0 => None,
            _ => Some(attributes),
        };
        Some(Cookie::new(name, value.to_string(), out_attributes))
    }

    /// convert a cookie to a string that can be sent in responses or requests
    pub fn to_string(&self) -> String {
        let mut out: String = format!("{}={}", self.name, self.value);
        if let Some(n) = &self.attributes {
            for attr in n {
                out.push_str(format!("; {}", attr.to_string()).as_str());
            }
        }

        out
    }
}
