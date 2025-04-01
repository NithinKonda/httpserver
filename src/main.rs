// main.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

trait RequestHandler {
    fn handle_request(&self, data: &[u8]) -> Vec<u8>;
}

struct TCPServer<T: RequestHandler> {
    host: String,
    port: u16,
    handler: T,
}

impl<T: RequestHandler> TCPServer<T> {
    fn new(host: String, port: u16, handler: T) -> Self {
        TCPServer { host, port, handler }
    }

    fn start(&self) -> std::io::Result<()> {
        let address = format!("{}:{}", self.host, self.port);
        
        let listener = TcpListener::bind(&address)?;

        
        println!("Listening at {}", address);
        

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peer_addr = stream.peer_addr()?;
                    println!("Connected by {}", peer_addr);
                    

                    self.handle_client(stream)?;
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {

        let mut buffer = [0; 1024];
        
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read > 0 {
            let request_data = &buffer[0..bytes_read];
            let response = self.handler.handle_request(request_data);
            
            stream.write_all(&response)?;
        }

        Ok(())
    }

}

struct EchoHandler;

impl RequestHandler for EchoHandler {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}


struct HTTPHandler;

impl HTTPHandler {
    fn new() -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert("Server".to_string(), "CrudeServer".to_string());
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        let mut status_codes = std::collections::HashMap::new();
        status_codes.insert(200, "OK".to_string());
        status_codes.insert(404, "Not Found".to_string());
        
        HTTPHandler {
            headers,
            status_codes,
        }
    }

    fn response_line(&self, status_code: u16) -> Vec<u8> {
        let reason = self.status_codes.get(&status_code).unwrap_or(&"Unknown".to_string());
        format!("HTTP/1.1 {} {}\r\n", status_code, reason).into_bytes()
    }

    fn response_headers(&self, extra_headers: Option<std::collections::HashMap<String, String>>) -> Vec<u8> {

        let mut headers_copy = self.headers.clone();

        if let Some(extra) = extra_headers {
            for (key, value) in extra {
                headers_copy.insert(key, value);
            }
        }

        let mut result = Vec::new();
        for (header, value) in &headers_copy {
            let header_line = format!("{}: {}\r\n", header, value);
            result.extend_from_slice(header_line.as_bytes());
        }
        
        result
    }
}

impl RequestHandler for HTTPHandler {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        let response_line = self.response_line(200);
        let response_headers = self.response_headers(None);
        let blank_line = b"\r\n";
        let response_body = br#"
            <html>
                <body>
                    <h1>Request received!</h1>
                </body>
            </html>
        "#;
        
        let mut response = Vec::new();
        response.extend_from_slice(&response_line);
        response.extend_from_slice(&response_headers);
        response.extend_from_slice(blank_line);
        response.extend_from_slice(response_body);
        
        response
    }
}

fn main() -> std::io::Result<()> {
    let handler = HTTPHandler;

    let server = TCPServer::new("127.0.0.1".to_string(), 8888, handler);

    server.start()
}