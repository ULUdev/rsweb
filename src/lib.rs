//! # rsweb
//! ## library for interacting with the http protocol and creating a multithreaded web server
//! To create a simple server just use the following code:
//! ```rust
//!
//! use rsweb::resource::ResourceLoader;
//! use rsweb::route::Router;
//! use rsweb::server::Server;
//! use rsweb::config::Config;
//!
//! let conf = Config {http: None, ssl: None}; // just a config so this example works. In reality you would load a config
//! let mut server = Server::new(
//!     10, // number of threads
//!     ResourceLoader::new(10, ".".to_string(), true), // create a new resource loader with capacity 10
//!     // and caching enabled
//!     Router::new(String::from("/index.html")), // create a new router with index at index.html
//!     8080, // port
//!     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)), // ip (localhost in this case)
//!     conf,
//! );
//! ```
//!
//! `rsweb` also supports ssl. To create a simple server that uses ssl use:
//! ```rust
//! use rsweb::resource::ResourceLoader;
//! use rsweb::route::Router;
//! use rsweb::ssl::SSLServer;
//!
//! let mut server = SSLServer::new(
//!     10, // number of threads
//!     ResourceLoader::new(10, ".".to_string(), true), // create a new resource loader with capacity 10 and caching enabled
//!     Router::new(String::from("/index.html")), // create a new router with index at index.html
//!     8080, // port
//!     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)), // ip (localhost in this case)
//!     String::from("key.pem"), // private key file
//!     String::from("certs.pem"), // certificate chain file
//! );
//! ```

/// the cli of `rsweb`
pub mod cli;

/// the configuration reading functionality of `rsweb`
pub mod config;

/// a dynamic buffer implementation used to read from streams
pub mod dbuffer;

/// errors for `rsweb`
pub mod error;
pub mod http;

/// logging functions for `rsweb`
pub mod log;

/// resource handler and cache storage
pub mod resource;

/// router for requests
pub mod route;

/// basic HTTP server implementation
pub mod server;

/// basic HTTPS server implementation
pub mod ssl;

/// Threadpool implementation included in the logs produced by [`log`]
pub mod tp;

pub use tp::ThreadPool;
/// version str of rsweb. Used for logging and CLI
pub const RSWEB_VERSION: &str = "0.8.9";
/// version str of rsweb used in the `Server` response header
pub const RSWEB_SERVER_STR: &str = "rsweb/0.8.9";

#[cfg(test)]
mod tests {
    mod thread_pool {
	use crate::tp::ThreadPool;
	#[test]
	fn thread_pool_create() {
    	    let _ = ThreadPool::new(1);
	}
	
	#[test]
	#[should_panic]
	fn empty_thread_pool() {
            let _ = ThreadPool::new(0);
	}
    }

    mod dbuffer {
	use crate::dbuffer::DBuffer;

	#[test]
	fn dbuffer_create() {
    	    let _ = DBuffer::new();
	}

	#[test]
	fn dbuffer_create_with_cap() {
	    let _ = DBuffer::with_capacity(6);
	}

	#[test]
	fn dbuffer_read() {
	    let mut dbuffer = DBuffer::new();
	    let s = String::from("hello");
	    assert!(dbuffer.read_until_zero(&mut s.as_bytes()).is_ok());
	}

	#[test]
	fn dbuffer_to_string() {
	    let mut dbuffer = DBuffer::new();
	    let s = String::from("hello");
	    assert!(dbuffer.read_until_zero(&mut s.as_bytes()).is_ok());
	    assert!(dbuffer.to_string().is_ok());
	}
    }
}
