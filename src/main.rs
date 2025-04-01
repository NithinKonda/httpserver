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

    fn start(&self) -> std::io::Result<()>{
            let address = format!("{}:{}", self.host, self.port);

            let listener = TcpListener::bind(&address)?;

            println!("Listening in {}", address);


            for stream in listener.incoming(){
                match stream {
                    Ok(stream) => {
                        let peer_addr = stream.peer_addr()?;
                        println!("Connection by {}", peer_addr);

                        self.handle_client(stream)?;
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
            Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}
