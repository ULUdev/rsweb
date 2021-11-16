//! # rsweb
//! ## library for interacting with the http protocol and creating a multithreaded web server

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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn empty_thread_pool() {
        let _ = ThreadPool::new(0);
    }
}
