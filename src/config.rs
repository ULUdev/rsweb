use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Deserialize)]
pub struct Config {
    pub port: usize,
    pub ip: String,
    pub threads: Option<usize>,
    pub ressources: Ressource,
    pub ssl: Option<SslConfig>,
    pub logfile: Option<String>,
    pub ressource_cache: Option<bool>,
    pub cache_capacity: Option<usize>,
}

#[derive(Deserialize)]
pub struct SslConfig {
    pub private_key: String,
    pub certificate_chain: String,
}

#[derive(Deserialize)]
pub struct Ressource {
    pub root: String,
    pub index: Option<String>,
    pub routes: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
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
