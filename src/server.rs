use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address);
        match listener {
            Err(e) => panic!("Error creating TcpListener for Server: {}", e),
            Ok(l) => {
                println!("Server is listening on {}", &self.address);
                self.handle_connections(&l);
            }
        }
    }

    fn handle_connections(&self, listener: &TcpListener) {
        loop {
            match listener.accept() {
                Err(e) => println!("Error accepting incoming connection: {}", e),
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Err(e) => println!("Failed to read from connection: {}", e),
                        Ok(_) => {
                            println!("Received request:\n{}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    println!("Request parsed:\n{}", req);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>It Works!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Error parsing request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                            ()
                        }
                    }
                }
            }
        }
    }
}
