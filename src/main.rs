use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


struct TcpServer{
    host: String,
    port: u16,
}


impl TcpServer{
    fn new(host:String, port :u16)->Self {
        TcpServer { host, port }
    }

    fn start(&self){
        
    }
}


fn main() {
    println!("Hello, world!");
}
