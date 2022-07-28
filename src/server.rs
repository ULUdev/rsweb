#![allow(unused_assignments)]
use crate::config;
use crate::dbuffer::DBuffer;
use crate::http::MimeType;
use crate::http::{body::*, header::*, request::*, response::*, StatusCode};
use crate::log;
use crate::resource::ResourceLoader;
use crate::route::*;
use crate::ThreadPool;
use crate::RSWEB_SERVER_STR;
use crate::RSWEB_VERSION;
use crate::{error, msg};
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
    config: config::Config,
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
        config: config::Config,
    ) -> Server {
        Server {
            tp: ThreadPool::new(capacity),
            rl,
            router,
            port,
            ip,
            config,
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
        let _ = logger.set_term(btui::Terminal::default());
        let _ = logger.set_logfile(lf);
        msg!(logger, "starting HTTP server (rsweb {})", RSWEB_VERSION);
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // TODO: make the executing thread mutate the resource loader of the main
                    // thread
                    let router = self.router.clone();
                    let logfile = lf.to_string();
                    let mut resload = self.rl.clone();
                    let config = self.config.clone();
                    self.tp.execute(move || {
                        let mut logging = log::Logger::new();
                        logging.set_term(btui::Terminal::new());
                        let _ = logging.set_logfile(logfile.as_str());
                        let mut buf = DBuffer::new();
                        if let Err(_) = buf.read_http_request(&mut stream) {
                            error!(logging, "failed to read from stream");
                        }
                        // TODO: only take the headers as string. The body might be non UTF-8
                        let data: String = match buf.to_string() {
                            Ok(n) => n,
                            Err(_) => {
                                error!(logging, "failed to parse data to utf8");
                                return;
                            }
                        };
                        if let Ok(req) = HTTPRequest::from_string(data) {
                            msg!(logging, "request: {} {}", req.get_method(), req.get_path());
                            if let Some(n) = router.lookup(req.get_path()) {
                                let resp = match n {
                                    Route::Route(p) => p,
                                    Route::Alias(q) => {
                                        let mut headers = vec![HTTPResponseHeaders::Server(
                                            RSWEB_SERVER_STR.to_string(),
                                        )];
                                        let mut status = StatusCode::InternalServerError;
                                        let mut body = Body::new(String::new());
                                        match resload.load(q[1..].to_string()) {
                                            Some(n) => {
                                                headers.push(HTTPResponseHeaders::ContentType(
                                                    n.get_mime(),
                                                ));
                                                body = Body::from_bytes(n.get_content());
                                                status = StatusCode::Ok;
                                            }
                                            None => {
                                                headers.push(HTTPResponseHeaders::ContentType(
                                                    MimeType::Html,
                                                ));
                                                status = StatusCode::NotFound;
						body = match config.http {
						    Some(http_conf) => {
							match http_conf.resources.notfound_page {
							    Some(page) => {
								match resload.load(page[1..].to_string()) {
								    Some(n) => Body::from_bytes(n.get_content()),
								    None => Body::new(String::from("<h1>494 Not Found</h1>"))
								}
							    },
							    None => Body::new(String::from("<h1>404 Not Found</h1>"))
							}
						    },
						    None => Body::new(String::from("<h1>404 Not Found</h1>"))
						};
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
                                    Err(_) => error!(logging, "failed to write to stream"),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => error!(logging, "failed to flush stream"),
                                }
                            } else {
                                let mut header =
                                    vec![HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string())];
                                let mut body = Body::new(String::new());
                                let mut status = StatusCode::InternalServerError;
                                match resload.load(req.get_path()[1..].to_string()) {
                                    Some(n) => {
                                        header.push(HTTPResponseHeaders::ContentType(n.get_mime()));
                                        body = Body::from_bytes(n.get_content());
                                        status = StatusCode::Ok;
                                    }
                                    None => {
                                        header
                                            .push(HTTPResponseHeaders::ContentType(MimeType::Html));
                                        status = StatusCode::NotFound;
					body = match config.http {
					    Some(http_conf) => {
						match http_conf.resources.notfound_page {
						    Some(page) => {
							match resload.load(page[1..].to_string()) {
							    Some(n) => Body::from_bytes(n.get_content()),
							    None => Body::new(String::from("<h1>494 Not Found</h1>"))
							}
						    },
						    None => Body::new(String::from("<h1>404 Not Found</h1>"))
						}
					    },
					    None => Body::new(String::from("<h1>404 Not Found</h1>"))
					};
                                    }
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
                                    Err(_) => error!(logging, "failed to write to stream"),
                                }
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(_) => error!(logging, "failed to flush stream"),
                                }
                            }
                        } else {
                            error!(logging, "failed to parse request");
                        }
                        match stream.shutdown(std::net::Shutdown::Both) {
                            Ok(_) => (),
                            Err(_) => error!(logging, "failed to shutdown stream"),
                        }
                    });
                }
                Err(_) => (),
            }
        }

        Ok(())
    }
}

/// server using a function to deal with requests
pub struct FuncServer {
    tp: ThreadPool,
    port: usize,
    ip: IpAddr,
    logfile: String,
}

impl FuncServer {
    /// create a new server
    /// # Arguments
    /// * `capacity`: the capacity of the thread pool
    /// * `port`: the port to use
    /// * `ip`: the ip address to bind to
    /// * `logfile`: the logfile to use
    pub fn new(capacity: usize, port: usize, ip: IpAddr, logfile: &str) -> FuncServer {
        FuncServer {
            tp: ThreadPool::new(capacity),
            port,
            ip,
            logfile: logfile.to_string(),
        }
    }

    /// run the server using `func` as the function
    pub fn run<F: 'static>(&mut self, func: F) -> Result<(), std::io::Error>
    where
        F: FnOnce(HTTPRequest) -> HTTPResponse + std::marker::Send + Copy,
    {
        let listener = match self.ip {
            addr => match TcpListener::bind(format!("{}:{}", addr, self.port)) {
                Ok(n) => n,
                Err(e) => return Err(e),
            },
        };

        let mut logger = log::Logger::new();
        let _ = logger.set_logfile(self.logfile.as_str());
        let _ = logger.set_term(btui::Terminal::new());
        msg!(logger, "starting HTTP server (rsweb {})", RSWEB_VERSION);
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut log = log::Logger::new();
                    let _ = log.set_logfile(self.logfile.as_str());
                    let _ = log.set_term(btui::Terminal::new());
                    self.tp.execute(move || {
                        let mut buf = DBuffer::new();
                        if let Err(_) = buf.read_http_request(&mut stream) {
                            error!(log, "failed to read from stream");
                            return;
                        }
                        if let Ok(data) = buf.to_string() {
                            match HTTPRequest::from_string(data) {
                                Ok(req) => {
                                    let resp = func(req);
                                    if let Err(_) = stream.write(&resp.to_bytes()) {
                                        error!(log, "failed to write to stream");
                                    }
                                    if let Err(_) = stream.flush() {
                                        error!(log, "failed to flush stream");
                                    }
                                }
                                Err(_) => error!(log, "failed to parse HTTP request"),
                            }
                        }
                    });
                }
                Err(e) => error!(logger, "{}", e),
            }
        }
        Ok(())
    }
}
