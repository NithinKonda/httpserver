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

fn main() -> std::io::Result<()> {
    let handler = EchoHandler;
    let server = TCPServer::new("127.0.0.1".to_string(), 8888, handler);
    server.start()
}