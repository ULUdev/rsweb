//! # rsweb
//! ## library for interacting with the http protocol and creating a multithreaded web server
//! To create a simple server just use the following code:
//! ```rust
//!
//! use rsweb::resource::ResourceLoader;
//! use rsweb::route::Router;
//! use rsweb::server::Server;
//!
//! let mut server = Server::new(
//!     10, // number of threads
//!     ResourceLoader::new(10, ".".to_string()), // create a new resource loader with capacity 10
//!     Router::new(String::from("/index.html")), // create a new router with index at index.html
//!     8080, // port
//!     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)), // ip (localhost in this case)
//! );
//! ```
//!
//! `rsweb` also supports ssl. To create a simple server that uses ssl use:
//! ```rust
//! use rsweb::resource::ResourceLoader;
//! use rsweb::route::Router;
//! use rsweb::ssl::SSLServer
//!
//! let mut server = SSLServer::new(
//!     10, // number of threads
//!     ResourceLoader::new(10, ".".to_string()), // create a new resource loader with capacity 10
//!     Router::new(String::from("/index.html")), // create a new router with index at index.html
//!     8080, // port
//!     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)), // ip (localhost in this case)
//!     String::from("key.pem"), // private key file
//!     String::from("certs.pem"), // certificate chain file
//! );
//! ```

pub mod cli;
pub mod config;
pub mod dbuffer;
pub mod error;
pub mod http;
pub mod log;
pub mod resource;
pub mod route;
pub mod server;
pub mod ssl;
pub mod tp;

pub use tp::ThreadPool;
pub const RSWEB_VERSION: &str = "0.5.5";

#[cfg(test)]
mod tests {
    use super::tp::ThreadPool;

    #[test]
    #[should_panic]
    fn empty_thread_pool() {
        let _ = ThreadPool::new(0);
    }
}
