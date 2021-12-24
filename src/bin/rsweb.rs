use rsweb::config::{load_config, Config};
use rsweb::resource::ResourceLoader;
use rsweb::route::Router;
use rsweb::server::Server;
use rsweb::ssl::SSLServer;
use std::env::args;
use std::process::exit;
use std::str::FromStr;

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
    let index_page = conf
        .resources
        .index
        .unwrap_or_else(|| String::from("/index.html"));
    let mut router = Router::new(index_page);
    let threads: usize = conf.threads.unwrap_or(4);
    let port: usize = conf.port;
    let logfile: String = conf.logfile.unwrap_or_else(|| String::from("log.txt"));
    let use_cache: bool = conf.resource_cache.unwrap_or(true);
    let cache_cap: usize = conf.cache_capacity.unwrap_or(10);
    let routes: Vec<(String, String)> = conf
        .resources
        .routes
        .unwrap_or(Vec::new())
        .iter()
        .map(|x| {
            let mut parts = x.split(':');
            let lh = parts.next().unwrap_or("");
            let rh = parts.next().unwrap_or("");
            (lh.to_string(), rh.to_string())
        })
        .filter(|x| {
            let (lh, rh) = x;
            !lh.is_empty() && !rh.is_empty()
        })
        .collect();
    for (lh, rh) in routes {
        router.route(lh, rh);
    }
    let aliases: Vec<(String, String)> = conf
        .resources
        .aliases
        .unwrap_or(Vec::new())
        .iter()
        .map(|x| {
            let mut parts = x.split(':');
            let lh = parts.next().unwrap_or("");
            let rh = parts.next().unwrap_or("");
            (lh.to_string(), rh.to_string())
        })
        .filter(|x| {
            let (lh, rh) = x;
            !lh.is_empty() && !rh.is_empty()
        })
        .collect();
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
    //         ResourceLoader::new(10, ".".to_string()),
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
                ResourceLoader::new(cache_cap, ".".to_string(), use_cache),
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
            match server.run(logfile.as_str()) {
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
                ResourceLoader::new(cache_cap, ".".to_string(), use_cache),
                router.clone(),
                port,
                addr,
            );
            match server.run(logfile.as_str()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("error at runtime: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }
}
