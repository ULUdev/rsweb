use crate::http::header::HTTPResponseHeaders;
use crate::http::response::HTTPResponse;
use crate::http::Body;
use crate::http::StatusCode;
use crate::RSWEB_SERVER_STR;
use std::collections::HashMap;
use wildmatch::WildMatch;

/// an enum for the two types of routes:
///
/// - `Route`: returns a redirect (status 301) to the actual address
/// - `Alias`: opens a different file than the one requested (e.g if `/` was requested that might
/// be an alias to `/index`)
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
    #[allow(unused_variables)]
    pub fn new(index: String) -> Router {
        Router {
            routemap: HashMap::new(),
            aliasmap: HashMap::new(),
        }
    }

    /// add a new route
    pub fn route(&mut self, from: String, to: String) {
        self.routemap.insert(from, to);
    }

    /// add a new alias
    pub fn alias(&mut self, key: String, alias: String) {
        self.aliasmap.insert(key, alias);
    }

    /// lookup and return a response if a route was found.
    /// If `pattern` matches multiple keys the first one found gets returned
    pub fn lookup(&self, pattern: String) -> Option<Route> {
        let mut resp: Option<Route> = None;
        for key in self.routemap.keys() {
            if WildMatch::new(key.as_str()).matches(pattern.as_str()) {
                let body = Body::new(String::new());
                let loc = self.routemap.get(key.as_str()).unwrap();
                resp = Some(Route::Route(HTTPResponse::new(
                    StatusCode::MovedPermanently,
                    vec![
                        HTTPResponseHeaders::Location(loc.to_string()),
                        HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string()),
                    ],
                    body,
                )));
                break;
            }
        }
        if resp.is_none() {
            for alias in self.aliasmap.keys() {
                if WildMatch::new(alias.as_str()).matches(pattern.as_str()) {
                    resp = Some(Route::Alias(self.aliasmap.get(alias).unwrap().to_string()));
                    break;
                }
            }
        }
        resp
    }
}
