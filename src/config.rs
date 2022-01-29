use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Deserialize)]
pub struct Config {
    pub http: Option<HTTPConfig>,
    pub ssl: Option<SslConfig>,
}

#[derive(Deserialize)]
pub struct HTTPConfig {
    pub port: usize,
    pub ip: String,
    pub threads: Option<usize>,
    pub resources: Resource,
    pub logfile: Option<String>,
    pub allowed_methods: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct SslConfig {
    pub private_key: String,
    pub certificate_chain: String,
    pub port: usize,
    pub ip: String,
    pub threads: Option<usize>,
    pub resources: Resource,
    pub logfile: Option<String>,
    pub allowed_methods: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Resource {
    pub root: String,
    pub index: Option<String>,
    pub routes: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub resource_cache: Option<bool>,
    pub cache_capacity: Option<usize>,
}

/// load a config from a file
pub fn load_config(path: &str) -> Result<Config, Error> {
    let contents: String = match read_to_string(path) {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    match toml::from_str(contents.as_str()) {
        Ok(n) => Ok(n),
        Err(e) => Err(Error::new(
            std::io::ErrorKind::Other,
            format!("failed to parse config file: {}", e),
        )),
    }
}
