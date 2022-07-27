#![allow(unused_assignments)]
use crate::dbuffer::DBuffer;
use crate::error::ServerError;
use crate::http::body::Body;
use crate::http::header::HTTPResponseHeaders;
use crate::http::request::HTTPMethod;
use crate::http::request::HTTPRequest;
use crate::http::response::HTTPResponse;
use crate::http::MimeType;
use crate::http::StatusCode;
use crate::log;
use crate::resource::ResourceLoader;
use crate::route::*;
use crate::ThreadPool;
use crate::RSWEB_SERVER_STR;
use crate::RSWEB_VERSION;
use crate::{error, msg};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpListener;
use std::sync::Arc;

/// an SSL/TLS server using a resource loader and router
pub struct SSLServer {
    tp: ThreadPool,
    rl: ResourceLoader,
    port: usize,
    ip: IpAddr,
    router: Router,
    sslacceptor: Arc<SslAcceptor>,
}

impl SSLServer {
    /// create a new SSL server
    pub fn new(
        capacity: usize,
        rl: ResourceLoader,
        router: Router,
        port: usize,
        ip: IpAddr,
        privkeyfile: String,
        certchainfile: String,
    ) -> Result<SSLServer, ServerError> {
        let mut acceptor = match SslAcceptor::mozilla_intermediate(SslMethod::tls()) {
            Ok(n) => n,
            Err(_) => {
                return Err(ServerError::new("error creating SSL Acceptor"));
            }
        };
        match acceptor.set_private_key_file(privkeyfile.as_str(), SslFiletype::PEM) {
            Ok(n) => n,
            Err(_) => {
                return Err(ServerError::new("error creating SSL Acceptor"));
            }
        }
        match acceptor.set_certificate_chain_file(certchainfile.as_str()) {
            Ok(n) => n,
            Err(_) => {
                return Err(ServerError::new("error creating SSL Acceptor"));
            }
        }
        match acceptor.check_private_key() {
            Ok(n) => n,
            Err(_) => {
                return Err(ServerError::new(
                    format!("problem with private key: '{}'", privkeyfile).as_str(),
                ));
            }
        }
        let acceptor = Arc::new(acceptor.build());
        Ok(SSLServer {
            tp: ThreadPool::new(capacity),
            rl,
            router,
            port,
            ip,
            sslacceptor: acceptor,
        })
    }

    /// run the SSL server
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
            error!(logger, "couldn't open log file");
        }
        msg!(logger, "starting HTTPS server (rsweb {})", RSWEB_VERSION);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let acceptor = self.sslacceptor.clone();
                    let mut resload = self.rl.clone();
                    let router = self.router.clone();
                    let logfile = lf.to_string();

                    self.tp.execute(move || {
                        let mut logging = log::Logger::new();
                        logging.set_term(btui::Terminal::new());
                        let _ = logging.set_logfile(logfile.as_str());
                        let mut buf = DBuffer::new();
                        let mut stream = match acceptor.accept(stream) {
                            Ok(n) => n,
                            Err(_) => {
                                error!(logging, "failed SSL handshake");
                                return;
                            }
                        };
                        if let Err(_) = buf.read_http_request(&mut stream) {
                            error!(logging, "failed to read from stream");
                        }
                        let data: String = match buf.to_string() {
                            Ok(n) => n,
                            Err(_) => {
                                error!(logging, "failed to parse data to utf8");
                                String::new()
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
                                        let mut body = Body::new(String::new());
                                        let mut status = StatusCode::InternalServerError;
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
                                                body = Body::new(String::from(
                                                    "<h1>404 not found</h1>",
                                                ));
                                                status = StatusCode::NotFound;
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
                                let mut headers =
                                    vec![HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string())];
                                let mut body = Body::new(String::new());
                                let mut status = StatusCode::InternalServerError;
                                match resload.load(req.get_path()[1..].to_string()) {
                                    Some(n) => {
                                        headers
                                            .push(HTTPResponseHeaders::ContentType(n.get_mime()));
                                        body = Body::from_bytes(n.get_content());
                                        status = StatusCode::Ok;
                                    }
                                    None => {
                                        headers
                                            .push(HTTPResponseHeaders::ContentType(MimeType::Html));
                                        body = Body::new(String::from("<h1>404 not found</h1>"));
                                        status = StatusCode::NotFound;
                                    }
                                }
                                headers.push(HTTPResponseHeaders::ContentLength(
                                    body.get_bytes().len(),
                                ));
                                if req.get_method() == HTTPMethod::Head {
                                    body = Body::new(String::new());
                                }
                                let response = HTTPResponse::new(status, headers, body);
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
                        match stream.shutdown() {
                            Ok(_) => (),
                            Err(_) => error!(logging, "failed to shutdown stream"),
                        }
                    });
                }
                Err(_) => {
                    error!(logger, "connection failed");
                }
            }
        }
        Ok(())
    }
}

/// server using a function to deal with requests
pub struct SSLFuncServer {
    tp: ThreadPool,
    port: usize,
    ip: IpAddr,
    logfile: String,
    acceptor: Arc<SslAcceptor>,
}

impl SSLFuncServer {
    /// create a new server that uses SSL
    /// # Arguments
    /// * `capacity`: the capacity of the thread pool
    /// * `port`: the port to use
    /// * `ip`: the ip address to bind to
    /// * `logfile`: the logfile to use
    /// * `privkeyfile`: file that contains the private key
    /// * `certchainfile`: file that contains a certificate chain (or single certificate)
    pub fn new(
        capacity: usize,
        port: usize,
        ip: IpAddr,
        logfile: &str,
        privkeyfile: &str,
        certchainfile: &str,
    ) -> Result<SSLFuncServer, ServerError> {
        let mut acceptor = match SslAcceptor::mozilla_intermediate(SslMethod::tls()) {
            Ok(n) => n,
            Err(_) => {
                return Err(ServerError::new("error creating SSL Acceptor"));
            }
        };
        match acceptor.set_private_key_file(privkeyfile, SslFiletype::PEM) {
            Ok(_) => (),
            Err(_) => {
                return Err(ServerError::new("error setting private key file"));
            }
        }
        match acceptor.set_certificate_chain_file(certchainfile) {
            Ok(_) => (),
            Err(_) => {
                return Err(ServerError::new("error setting certificate chain file"));
            }
        }
        match acceptor.check_private_key() {
            Ok(_) => (),
            Err(_) => {
                return Err(ServerError::new(
                    format!("problem with private key: '{}'", privkeyfile).as_str(),
                ));
            }
        }
        let acceptor = Arc::new(acceptor.build());
        Ok(SSLFuncServer {
            tp: ThreadPool::new(capacity),
            port,
            ip,
            logfile: logfile.to_string(),
            acceptor,
        })
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
                Ok(stream) => {
                    let mut log = log::Logger::new();
                    let _ = log.set_logfile(self.logfile.as_str());
                    let _ = log.set_term(btui::Terminal::new());
                    let acceptor = self.acceptor.clone();
                    self.tp.execute(move || {
                        let mut stream = match acceptor.accept(stream) {
                            Ok(n) => n,
                            Err(_) => {
                                error!(log, "failed SSL handshake");
                                return;
                            }
                        };
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
                                Err(_) => {
                                    error!(log, "failed to parse HTTP request")
                                }
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
