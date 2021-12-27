use crate::dbuffer::DBuffer;
use crate::error::ServerError;
use crate::http::header::HTTPResponseHeaders;
use crate::http::request::HTTPRequest;
use crate::http::response::HTTPResponse;
use crate::http::MimeType;
use crate::http::*;
use crate::log;
use crate::resource::ResourceLoader;
use crate::route::*;
use crate::ThreadPool;
use crate::RSWEB_SERVER_STR;
use crate::RSWEB_VERSION;
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
            logger.log("couldn't open log file", log::LogType::Error);
        }
        logger.log(format!("starting server (rsweb {})", RSWEB_VERSION), log::LogType::Log);
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
                                logging.log("failed SSL handshake", log::LogType::Error);
                                return;
                            }
                        };
                        if let Err(_) = buf.read_http_request(&mut stream) {
                            logging.log("failed to read from stream", log::LogType::Error);
                        }
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
                                        let (response_body, mime_type) =
                                            resload.load(q[1..].to_string());
                                        let header = vec![
                                            HTTPResponseHeaders::ContentType(mime_type),
                                            HTTPResponseHeaders::Server(
                                                RSWEB_SERVER_STR.to_string(),
                                            ),
                                        ];
                                        let mut response = HTTPResponse::new(
                                            StatusCode::Ok,
                                            header,
                                            Body::new(response_body.clone()),
                                        );
                                        if response_body.is_empty() {
                                            response = HTTPResponse::new(
                                                StatusCode::NotFound,
                                                vec![
                                                    HTTPResponseHeaders::ContentType(
                                                        MimeType::Html,
                                                    ),
                                                    HTTPResponseHeaders::Server(
                                                        RSWEB_SERVER_STR.to_string(),
                                                    ),
                                                ],
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
                                let header = vec![
                                    HTTPResponseHeaders::ContentType(mime_type),
                                    HTTPResponseHeaders::Server(RSWEB_SERVER_STR.to_string()),
                                ];
                                let mut response = HTTPResponse::new(
                                    StatusCode::Ok,
                                    header,
                                    Body::new(response_body.clone()),
                                );
                                if response_body.is_empty() {
                                    response = HTTPResponse::new(
                                        StatusCode::NotFound,
                                        vec![
                                            HTTPResponseHeaders::ContentType(MimeType::Html),
                                            HTTPResponseHeaders::Server(
                                                RSWEB_SERVER_STR.to_string(),
                                            ),
                                        ],
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
                        } else {
                            logging.log("failed to parse request", log::LogType::Error);
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
