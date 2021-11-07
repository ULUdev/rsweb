//! # HTTP
//! module containing functions to parse the http protocol
pub mod header;
pub mod parse;
pub mod body;
pub mod status;

pub use header::Header;
