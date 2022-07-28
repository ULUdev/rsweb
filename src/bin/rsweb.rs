use rsweb::cli::Arguments;
use rsweb::config::{load_config, Config};
use rsweb::resource::ResourceLoader;
use rsweb::route::Router;
use rsweb::server::Server;
use rsweb::ssl::SSLServer;
use std::process::exit;
use std::str::FromStr;
use std::thread;

fn main() {
    let arguments = Arguments::load();
    let path: String = arguments
        .configfile
        .unwrap_or_else(|| String::from("/etc/rsweb/rsweb.config.toml"));
    let conf: Config = match load_config(&path) {
        Ok(n) => n,
        Err(_) => {
            eprintln!("failed to parse config file. Exiting...");
            exit(1);
        }
    };
    let mut http: bool = false;
    let mut http_handle: Option<std::thread::JoinHandle<()>> = None;
    let mut ssl: bool = false;
    let mut ssl_handle: Option<std::thread::JoinHandle<()>> = None;
    if let Some(n) = conf.clone().http {
        http = true;
        let index_page = n
            .resources
            .index
            .unwrap_or_else(|| String::from("/index.html"));
        let mut router = Router::new(index_page);
        let threads: usize = n.threads.unwrap_or(4);
        let port: usize = n.port;
        let logfile: String = arguments.logfile.clone().unwrap_or_else(|| {
            n.logfile
                .unwrap_or_else(|| String::from("/var/log/rsweb/latest.log"))
        });
        let use_cache: bool = n.resources.resource_cache.unwrap_or(true);
        let cache_cap: usize = n.resources.cache_capacity.unwrap_or(10);
        let root: String = n.resources.root;
        let routes: Vec<(String, String)> = n
            .resources
            .routes
            .unwrap_or(Vec::new())
            .iter()
            .map(|x| {
                let lh = x.split_once(':').unwrap_or(("", "")).0;
                let rh = x.split_once(':').unwrap_or(("", "")).1;
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
        let aliases: Vec<(String, String)> = n
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
        let addr = match std::net::IpAddr::from_str(n.ip.as_str()) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("failed to parse ip address");
                exit(1);
            }
        };
        let mut server = Server::new(
            threads,
            ResourceLoader::new(cache_cap, root, use_cache),
            router.clone(),
            port,
            addr,
            conf.clone(),
        );
        if let Some(_) = conf.ssl {
            http_handle = Some(thread::spawn(move || match server.run(logfile.as_str()) {
                Ok(_) => (),
                Err(e) => eprintln!("runtime error: {}", e),
            }));
        } else {
            match server.run(logfile.as_str()) {
                Ok(_) => (),
                Err(e) => eprintln!("runtime error: {}", e),
            }
        }
    }

    if let Some(n) = conf.clone().ssl {
        ssl = true;
        if !http {
            // remember: there are people not using encryption out there
            eprintln!("warning: using an SSLServer without an HTTP server may lead to compatibility issues.");
        }
        let index_page = n
            .resources
            .index
            .unwrap_or_else(|| String::from("/index.html"));
        let mut router = Router::new(index_page);
        let threads: usize = n.threads.unwrap_or(4);
        let port: usize = n.port;
        let logfile: String = arguments.logfile.clone().unwrap_or_else(|| {
            n.logfile
                .unwrap_or_else(|| String::from("/var/log/rsweb/latest.log"))
        });
        let use_cache: bool = n.resources.resource_cache.unwrap_or(true);
        let cache_cap: usize = n.resources.cache_capacity.unwrap_or(10);
        let routes: Vec<(String, String)> = n
            .resources
            .routes
            .unwrap_or(Vec::new())
            .iter()
            .map(|x| {
                let lh = x.split_once(':').unwrap_or(("", "")).0;
                let rh = x.split_once(':').unwrap_or(("", "")).1;
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
        let aliases: Vec<(String, String)> = n
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
        let addr = match std::net::IpAddr::from_str(n.ip.as_str()) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("failed to parse ip address");
                exit(1);
            }
        };
        let privkey = n.private_key;
        let cert_chain = n.certificate_chain;
        let root = n.resources.root;
        let mut server = SSLServer::new(
            threads,
            ResourceLoader::new(cache_cap, root, use_cache),
            router,
            port,
            addr,
            privkey,
            cert_chain,
	    conf.clone(),
        )
        .unwrap_or_else(|_| {
            eprintln!("failed to create SSLServer. Exiting...");
            exit(1);
        });
        if http {
            ssl_handle = Some(thread::spawn(move || match server.run(logfile.as_str()) {
                Ok(_) => (),
                Err(e) => eprintln!("runtime error: {}", e),
            }));
        } else {
            match server.run(logfile.as_str()) {
                Ok(_) => (),
                Err(e) => eprintln!("runtime error: {}", e),
            }
        }
    }

    if !http && !ssl {
        eprintln!("config used doesn't specify any servers. exiting normally...");
        exit(0);
    }

    if let Some(handle) = http_handle {
        if let Err(_) = handle.join() {
            eprintln!("error joining threads");
        }
    }
    if let Some(handle) = ssl_handle {
        if let Err(_) = handle.join() {
            eprintln!("error joining threads");
        }
    }
}
