//! # rsweb
//! ## library for interacting with the http protocol and creating a multithreaded web server
//! To create a simple server just use the following code:
//! ```rust
//! 
//! use rsweb::ressource::RessourceLoader;
//! use rsweb::route::Router;
//! use rsweb::server::Server;
//!
//! let mut server = Server::new(
//!     10, // number of threads
//!     RessourceLoader::new(10, ".".to_string()), // create a new ressource loader with capacity 10
//!     Router::new(String::from("/index.html")), // create a new router with index at index.html
//!     8080, // port
//!     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)), // ip (localhost in this case)
//! );
//! ```

pub mod http;
pub mod log;
pub mod ressource;
pub mod route;
pub mod server;
pub mod tp;

pub use tp::ThreadPool;

#[cfg(test)]
mod tests {
    use super::tp::ThreadPool;

    #[test]
    #[should_panic]
    fn empty_thread_pool() {
        let _ = ThreadPool::new(0);
    }
}
