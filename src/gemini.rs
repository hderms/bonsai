use tokio::io::AsyncBufReadExt;

use tokio::io::BufReader;

use tokio::net::TcpStream;

use crate::util::crlf;

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Status {
    Input,
    SensitiveInput,
    Success,
    RedirectTemporary,
    RedirectPermanent,
    TemporaryFailure,
    ServerUnavailable,
    CgiError,
    ProxyError,
    SlowDown,
    PermanentFailure,
    NotFound,
    Gone,
    ProxyRequestRefused,
    BadRequest,
    ClientCertificateRequired,
    CertificateNotAuthorized,
    CertificateNotValid,
}
impl Status {
    fn numeric(self) -> u8 {
        match self {
            Status::Input => 10,
            Status::SensitiveInput => 11,
            Status::Success => 20,
            Status::RedirectTemporary => 30,
            Status::RedirectPermanent => 31,
            Status::TemporaryFailure => 40,
            Status::ServerUnavailable => 41,
            Status::CgiError => 42,
            Status::ProxyError => 43,
            Status::SlowDown => 44,
            Status::PermanentFailure => 50,
            Status::NotFound => 51,
            Status::Gone => 52,
            Status::ProxyRequestRefused => 53,
            Status::BadRequest => 59,
            Status::ClientCertificateRequired => 60,
            Status::CertificateNotAuthorized => 61,
            Status::CertificateNotValid => 62,
        }
    }
}
//<STATUS><SPACE><META><CR><LF>
pub struct ResponseHeader {
    status: Status,
    meta: String,
}

pub struct ResponseBody {
    body: Option<String>,
}
pub struct Response {
    response_header: ResponseHeader,
    response_body: ResponseBody,
}
impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(1024);
        let status_string = self.response_header.status.numeric().to_string();
        for char in status_string.chars() {
            vec.push(char as u8);
        }
        if !self.response_header.meta.is_empty() {
            vec.push(b' ');
        }

        for char in self.response_header.meta.chars() {
            vec.push(char as u8);
        }
        crlf(&mut vec);
        if let Some(body) = &self.response_body.body {
            for char in body.chars() {
                vec.push(char as u8);
            }
            crlf(&mut vec);
        }
        vec
    }
}

pub trait Server {
    fn process(&self, request: String) -> Response;
}
pub struct DefaultServer;
impl Server for DefaultServer {
    fn process(&self, _request: String) -> Response {
        Response {
            response_header: ResponseHeader {
                status: Status::Success,
                meta: String::from("this is a success"),
            },
            response_body: ResponseBody {
                body: Some(String::from("ok")),
            },
        }
    }
}

pub fn default_error() -> Response {
    Response {
        response_header: ResponseHeader {
            status: Status::TemporaryFailure,
            meta: String::from("Server Error"),
        },
        response_body: ResponseBody { body: None },
    }
}

pub async fn get_request_url(socket: &mut TcpStream) -> tokio::io::Result<String> {
    let mut reader = BufReader::new(socket);
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    Ok(line)
}
