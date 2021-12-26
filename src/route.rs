use crate::http::Header;
use crate::http::response::HTTPResponse;
use crate::http::Body;
use crate::http::StatusCode;
use std::collections::HashMap;

/// an http route
pub enum Route {
    Route(HTTPResponse),
    Alias(String),
}

/// a router for routing http traffic
#[derive(Clone)]
pub struct Router {
    routemap: HashMap<String, String>,
    aliasmap: HashMap<String, String>,
}

impl Router {
    /// create a new router with index page at `index`
    pub fn new(index: String) -> Router {
        let mut r = Router {
            routemap: HashMap::new(),
            aliasmap: HashMap::new(),
        };
        r.aliasmap.insert(String::from("/"), index);
        r
    }

    /// add a new route
    pub fn route(&mut self, from: String, to: String) {
        self.routemap.insert(from, to);
    }

    /// add a new alias
    pub fn alias(&mut self, key: String, alias: String) {
        self.aliasmap.insert(key, alias);
    }

    /// lookup and return a response if a route was found
    pub fn lookup(&self, key: String) -> Option<Route> {
        match self.routemap.get(&key) {
            Some(n) => {
                let mut header = Header::new(StatusCode::MovedPermanently);

                header.add_kv_pair(String::from("Location"), n.to_string());
                let body = Body::new(String::new());
                Some(Route::Route(HTTPResponse::new(header, body)))
            }
            None => self.aliasmap.get(&key).map(|n| Route::Alias(n.to_string())),
        }
    }
}
