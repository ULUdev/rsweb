use crate::dbuffer::DBuffer;
use crate::http::MimeType;
use crate::http::{body::*, header::*, request::*, response::*, StatusCode};
use crate::log;
use crate::resource::ResourceLoader;
use crate::route::*;
use crate::ThreadPool;
use crate::RSWEB_SERVER_STR;
use crate::RSWEB_VERSION;
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpListener;

/// a rsweb server using a resource loader and router
pub struct Server {
    tp: ThreadPool,
    rl: ResourceLoader,
    port: usize,
    ip: IpAddr,
    router: Router,
}

impl Server {
    /// create a new server
    /// # Arguments
    /// * `capacity`: the amount of threads to use
    /// * `rl`: the resource loader to use
    /// * `router`: the router to use
    /// * `port`: the port to use
    /// * `ip`: the ip address to run on
    pub fn new(
        capacity: usize,
        rl: ResourceLoader,
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
        if let Err(_) = logger.set_logfile(lf) {
            logger.log("couldn't open logfile", log::LogType::Error);
        }
        logger.log(
            format!("starting HTTP server (rsweb {})", RSWEB_VERSION),
            log::LogType::Log,
        );
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // TODO: make the executing thread mutate the resource loader of the main
                    // thread
                    let mut resload = self.rl.clone();
                    let router = self.router.clone();
                    let logfile = lf.to_string();
                    //let mut lg = logger.clone();
                    self.tp.execute(move || {
                        let mut logging = log::Logger::new();
                        logging.set_term(btui::Terminal::new());
                        let _ = logging.set_logfile(logfile.as_str());
                        let mut buf = DBuffer::new();
                        if let Err(_) = buf.read_http_request(&mut stream) {
                            logging.log("failed to read from stream", log::LogType::Error);
                        }
                        // TODO: only take the headers as string. The body might be non UTF-8
                        let data: String = match buf.to_string() {
                            Ok(n) => n,
                            Err(_) => {
                                logging.log("failed to parse data to utf8", log::LogType::Error);
                                String::new()
                            }
                        };
                        if let Ok(req) = HTTPRequest::from_string(data) {
                            logging.log(
                                format!("request: {} {}", req.get_method(), req.get_path()),
                                log::LogType::Log,
                            );
                            if let Some(n) = router.lookup(req.get_path()) {
                                let resp = match n {
                                    Route::Route(p) => p,
                                    Route::Alias(q) => {
                                        let mut headers = vec![HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string())];
                                        let mut status = StatusCode::InternalServerError;
                                        let mut body = Body::new(String::new());
                                        match resload.load(q[1..].to_string()) {
                                            Some(n) => {
                                                headers.push(HTTPResponseHeaders::ContentType(n.get_mime()));
                                                body = Body::from_bytes(n.get_content());
                                                status = StatusCode::Ok;
                                            },
                                            None => {
                                                headers.push(HTTPResponseHeaders::ContentType(MimeType::Html));
                                                status = StatusCode::NotFound;
                                                body = Body::new(String::from("<h1>404 not found</h1>"));
                                            }
                                        }
                                        headers.push(HTTPResponseHeaders::ContentLength(
                                            body.get_bytes().len(),
                                        ));
                                        if req.get_method() == HTTPMethod::Head {
                                            body = Body::new(String::new());
                                        }
                                        HTTPResponse::new(status, headers, body)
                                    }
                                };
                                match stream.write(&resp.to_bytes()) {
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
                                let mut header = vec![HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string())];
                                let mut body = Body::new(String::new());
                                let mut status = StatusCode::InternalServerError;
                                match resload.load(req.get_path()[1..].to_string()) {
                                    Some(n) => {
                                        header.push(HTTPResponseHeaders::ContentType(n.get_mime()));
                                        body = Body::from_bytes(n.get_content());
                                        status = StatusCode::Ok;
                                    },
                                    None => {
                                        header.push(HTTPResponseHeaders::ContentType(MimeType::Html));
                                        // TODO: load 404 page if known and otherwise use derfault
                                        body = Body::new(String::from("<h1>404 not found</h1>"));
                                        status = StatusCode::NotFound;
                                    },
                                }
                                header.push(HTTPResponseHeaders::ContentLength(
                                    body.get_bytes().len(),
                                ));
                                if req.get_method() == HTTPMethod::Head {
                                    body = Body::new(String::new());
                                }

                                let response = HTTPResponse::new(status, header, body);
                                match stream.write(&response.to_bytes()) {
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
                        } else {
                            logging.log("failed to parse request", log::LogType::Error);
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
