use crate::http::{body::*, header::*, request::*, response::*, StatusCode};
use crate::log;
use crate::ressource::RessourceLoader;
use crate::route::*;
use crate::ThreadPool;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::net::TcpListener;

/// a rsweb server using a ressource loader and router
pub struct Server {
    tp: ThreadPool,
    rl: RessourceLoader,
    port: usize,
    ip: IpAddr,
    router: Router,
}

impl Server {
    /// create a new server
    /// # Arguments
    /// * `capacity`: the amount of threads to use
    /// * `rl`: the ressource loader to use
    /// * `router`: the router to use
    /// * `port`: the port to use
    /// * `ip`: the ip address to run on
    pub fn new(
        capacity: usize,
        rl: RessourceLoader,
        router: Router,
        port: usize,
        ip: IpAddr,
    ) -> Server {
        Server {
            tp: ThreadPool::new(capacity),
            rl,
            router,
            port,
            ip,
        }
    }

    /// run the server
    /// # Arguments
    /// `lf`: the logfile to log to
    pub fn run(&mut self, lf: &str) -> Result<(), std::io::Error> {
        let listener = match self.ip {
            IpAddr::V4(addr) => match TcpListener::bind(format!("{}:{}", addr, self.port)) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            },
            IpAddr::V6(addr) => match TcpListener::bind(format!("{}:{}", addr, self.port)) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            },
        };

        let mut logger = log::Logger::new();
        logger.set_term(btui::Terminal::default());
        logger.log("starting server", log::LogType::Log);
        match logger.set_logfile(lf) {
            Ok(_) => (),
            Err(_) => {
                logger.log("couldn't set logfile", log::LogType::Error);
            }
        }
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut resload = self.rl.clone();
                    let router = self.router.clone();
                    let logfile = lf.to_string();
                    //let mut lg = logger.clone();
                    self.tp.execute(move || {
                        let mut logging = log::Logger::new();
                        logging.set_term(btui::Terminal::new());
                        let _ = logging.set_logfile(logfile.as_str());
                        let mut buf = [0u8; 1024];
                        let _ = stream.read(&mut buf);
                        let data: String = String::from(std::str::from_utf8(&buf).unwrap());
                        if let Ok(req) = HTTPRequest::from_string(data) {
                            logging.log(
                                format!("request: {} {}", req.get_method(), req.get_path()),
                                log::LogType::Log,
                            );
                            if let Some(n) = router.lookup(req.get_path()) {
                                let resp = match n {
                                    Route::Route(p) => p,
                                    Route::Alias(q) => {
                                        let (response_body, mime_type) =
                                            resload.load(q[1..].to_string());
                                        let mut header = Header::new(StatusCode::Ok);
                                        header.add_kv_pair(
                                            String::from("Content-Type"),
                                            mime_type.to_string(),
                                        );
                                        let mut response = HTTPResponse::new(
                                            header,
                                            Body::new(response_body.clone()),
                                        );
                                        if response_body.is_empty() {
                                            response = HTTPResponse::new(
                                                Header::new(StatusCode::NotFound),
                                                Body::new(String::from("<h1>404 not found</h1>")),
                                            );
                                        }
                                        response
                                    }
                                };
                                match stream.write(resp.to_string().as_str().as_bytes()) {
                                    Ok(_) => (),
                                    Err(_) => logging
                                        .log("failed to write to stream", log::LogType::Error),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => {
                                        logging.log("failed to flush stream", log::LogType::Error)
                                    }
                                }
                            } else {
                                let (response_body, mime_type) =
                                    resload.load(req.get_path()[1..].to_string());
                                let mut header = Header::new(StatusCode::Ok);
                                header.add_kv_pair(
                                    String::from("Content-Type"),
                                    mime_type.to_string(),
                                );
                                let mut response =
                                    HTTPResponse::new(header, Body::new(response_body.clone()));
                                if response_body.is_empty() {
                                    let mut header = Header::new(StatusCode::NotFound);
                                    header.add_kv_pair(
                                        String::from("Content-Type"),
                                        String::from("text/html"),
                                    );
                                    response = HTTPResponse::new(
                                        header,
                                        Body::new(String::from("<h1>404 not found</h1>")),
                                    );
                                }
                                match stream.write(response.to_string().as_str().as_bytes()) {
                                    Ok(_) => (),
                                    Err(_) => logging
                                        .log("failed to write to stream", log::LogType::Error),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => {
                                        logging.log("failed to flush stream", log::LogType::Error)
                                    }
                                }
                            }
                        }
                        match stream.shutdown(std::net::Shutdown::Both) {
                            Ok(_) => (),
                            Err(_) => logging.log("failed to shutdown stream", log::LogType::Error),
                        }
                    });
                }
                Err(_) => (),
            }
        }

        Ok(())
    }
}
