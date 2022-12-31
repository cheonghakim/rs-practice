use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self) {
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

                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {}
                                Err(err) => {
                                    println!("Fail to parse a request!: {}", err);
                                }
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
