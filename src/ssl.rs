use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use crate::log;
use crate::ressource::RessourceLoader;
use crate::route::*;
use crate::ThreadPool;
use crate::http::*;
use crate::http::request::HTTPRequest;
use crate::http::response::HTTPResponse;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::net::TcpListener;
use crate::error::ServerError;
use std::sync::Arc;

/// an SSL/TLS server using a ressource loader and router
pub struct SSLServer {
    tp: ThreadPool,
    rl: RessourceLoader,
    port: usize,
    ip: IpAddr,
    router: Router,
    sslacceptor: Arc<SslAcceptor>,
}

impl SSLServer {
    pub fn new(
        capacity: usize,
        rl: RessourceLoader,
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
                return Err(ServerError::new(format!("problem with private key: '{}'", privkeyfile).as_str()));
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
        let _ = logger.set_logfile(lf);
        logger.log("starting server", log::LogType::Log);
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
                        let mut buf = [0u8; 1024];
                        let mut stream = match acceptor.accept(stream) {
                            Ok(n) => n,
                            Err(_) => {
                                logging.log("failed SSL handshake", log::LogType::Error);
                                return;
                            }
                        };
                        match stream.ssl_read(&mut buf) {
                            Ok(_) => (),
                            Err(_) => {
                                logging.log("failed to read from SSL stream", log::LogType::Error);
                                return;
                            }
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
                        match stream.shutdown() {
                            Ok(_) => (),
                            Err(_) => logging.log("failed to shutdown stream", log::LogType::Error),
                        }
                    });
                }
                Err(_) => {
                    logger.log("connection failed", log::LogType::Error);
                }
            }
        }
        Ok(())
    }
}
