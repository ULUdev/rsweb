use rsweb::ressource::RessourceLoader;
use rsweb::route::Router;
use rsweb::server::Server;
use rsweb::ssl::SSLServer;
use std::io::Error;
use std::fs::read_to_string;
use std::process::exit;
use std::str::FromStr;
use serde_derive::Deserialize;
use std::env::args;

#[derive(Deserialize)]
pub struct Config {
    pub port: usize,
    pub ip: String,
    pub threads: Option<usize>,
    pub ressources: Ressource,
    pub ssl: Option<SslConfig>,
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

pub fn load_config(path: &str) -> Result<Config, Error> {
    let contents: String = match read_to_string(path) {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };
    match toml::from_str(contents.as_str()) {
        Ok(n) => Ok(n),
        Err(e) => Err(Error::new(std::io::ErrorKind::Other, format!("failed to parse config file: {}", e))),
    }
}

fn main() {
    let arguments: Vec<String> = args().collect();
    let mut path: &str = "/etc/rsweb/rsweb.config.toml";
    if arguments.len() >= 2 {
        path = arguments[1].as_str();
    }
    let conf: Config = match load_config(path) {
        Ok(n) => n,
        Err(_) => {
            eprintln!("failed to parse config file. Exiting...");
            exit(1);
        }
    };
    let index_page = conf.ressources.index.unwrap_or(String::from("/index.html"));
    let mut router = Router::new(index_page);
    let threads: usize = conf.threads.unwrap_or(4);
    let port: usize = conf.port;
    let routes: Vec<(String, String)> = conf.ressources.routes.unwrap_or(Vec::new()).iter().map(|x| {
        let mut parts = x.split(':');
        let lh = parts.next().unwrap_or("");
        let rh = parts.next().unwrap_or("");
        (lh.to_string(), rh.to_string())
    }).filter(|x| {
        let (lh, rh) = x;
        !lh.is_empty() && !rh.is_empty()
    }).collect();
    for (lh, rh) in routes {
        router.route(lh, rh);
    }
    let aliases: Vec<(String, String)> = conf.ressources.aliases.unwrap_or(Vec::new()).iter().map(|x| {
        let mut parts = x.split(':');
        let lh = parts.next().unwrap_or("");
        let rh = parts.next().unwrap_or("");
        (lh.to_string(), rh.to_string())
    }).filter(|x| {
        let (lh, rh) = x;
        !lh.is_empty() && !rh.is_empty()
    }).collect();
    for (lh, rh) in aliases {
        router.alias(lh, rh);
    }
    let addr = match std::net::IpAddr::from_str(conf.ip.as_str()) {
        Ok(n) => n,
        Err(_) => {
            eprintln!("failed to parse ip address");
            exit(1);
        }
    };
    // router.route(String::from("/test"), String::from("/test.html"));
    // if let Some(n) = conf.ssl {
    //     let mut server = SSLServer::new(
    //         threads,
    //         RessourceLoader::new(10, ".".to_string()),
    //         router,
    //         port,
    //         addr,
    //         n.private_key,
    //         n.certificate_chain,
    //     );
    // }
    match conf.ssl {
        Some(n) => {
            let mut server = match SSLServer::new(
                threads,
                RessourceLoader::new(10, ".".to_string()),
                router,
                port,
                addr,
                n.private_key,
                n.certificate_chain,
            ) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("failed to create server: {}", e);
                    std::process::exit(1);
                }
            };
            match server.run() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("error at runtime: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            let mut server = Server::new(
                threads,
                RessourceLoader::new(10, ".".to_string()),
                router.clone(),
                port,
                addr,
            );
            match server.run("log.txt") {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("error at runtime: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }
}
