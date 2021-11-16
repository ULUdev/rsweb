use crate::http::{body::*, header::*, request::*, response::*, StatusCode};
use crate::log;
use crate::ressource::RessourceLoader;
use crate::route::*;
use crate::ThreadPool;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::net::TcpListener;

pub struct Server {
    tp: ThreadPool,
    rl: RessourceLoader,
    port: usize,
    ip: IpAddr,
    router: Router,
}

impl Server {
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
                    //let mut lg = logger.clone();
                    self.tp.execute(move || {
                        let mut logging = log::Logger::new();
                        logging.set_term(btui::Terminal::new());
                        let mut buf = [0u8; 1024];
                        match stream.read(&mut buf) {
                            Ok(_) => (),
                            Err(_) => (),
                        };
                        let data: String = String::from(std::str::from_utf8(&buf).unwrap());
                        if let Ok(req) = HTTPRequest::from_string(data) {
                            logging.log(format!("request: {} {}", req.get_method(), req.get_path()), log::LogType::Log);
                            if let Some(n) = router.lookup(req.get_path()) {
                                let resp = match n {
                                    Route::Route(p) => p,
                                    Route::Alias(q) => {
                                        let response_body = resload.load(q[1..].to_string());
                                        let mut response = HTTPResponse::new(
                                            Header::new(StatusCode::Ok, Vec::new()),
                                            Body::new(response_body.clone()),
                                        );
                                        if response_body.is_empty() {
                                            response = HTTPResponse::new(
                                                Header::new(StatusCode::NotFound, Vec::new()),
                                                Body::new(String::from("<h1>404 not found</h1>")),
                                            );
                                        }
                                        response
                                    }
                                };
                                match stream.write(resp.to_string().as_str().as_bytes()) {
                                    Ok(_) => (),
                                    Err(_) => logging.log("failed to write to stream", log::LogType::Error),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => logging.log("failed to flush stream", log::LogType::Error),
                                }
                            } else {
                                let response_body = resload.load(req.get_path()[1..].to_string());
                                let mut response = HTTPResponse::new(
                                    Header::new(StatusCode::Ok, Vec::new()),
                                    Body::new(response_body.clone()),
                                );
                                if response_body.is_empty() {
                                    response = HTTPResponse::new(
                                        Header::new(StatusCode::NotFound, Vec::new()),
                                        Body::new(String::from("<h1>404 not found</h1>")),
                                    );
                                }
                                match stream.write(response.to_string().as_str().as_bytes()) {
                                    Ok(_) => (),
                                    Err(_) => logging.log("failed to write to stream", log::LogType::Error),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => logging.log("failed to flush stream", log::LogType::Error),
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
