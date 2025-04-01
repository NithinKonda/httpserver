use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};



trait RequestHandler {
    fn handle_request(&self, data : &[u8]) -> Vec<u8>;
}

struct TcpServer{
    host: String,
    port: u16,
    handler: T,
}


impl<T:RequestHandler> TcpServer<T>{
    fn new(host: String, port: u16, handler: T) -> Self {
        TCPServer { host, port, handler }
    }

    
}


fn main() {
    let server = TcpServer::new("127.0.0.1".to_string(), 8888);
    server.start();
}
