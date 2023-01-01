use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // 연결시도

        // 무한 반복
        loop {
            let res = listener.accept(); // 연결 시도

            // 연결에 대한 매칭
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    // 버퍼 선언
                    let mut buffer = [0; 1024];

                    // 버퍼에 대한 매칭
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // 버퍼를 utf 인코딩하여 콘솔에 표시
                            println!("Received a request!: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(err) => handler.handle_bad_request(&err),
                            };

                            if let Err(err) = response.send(&mut stream) {
                                println!("Failed to send response: {}", err);
                            }
                        }
                        Err(err) => {
                            println!("Failed To read from connection: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
