use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

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

struct HTTPRequest {
    method: Option<String>,
    uri: Option<String>,
    http_version: String,
}

impl HTTPRequest {
    fn new(data: &[u8]) -> Self {
        let mut request = HTTPRequest {
            method: None,
            uri: None,
            http_version: "1.1".to_string(),
        };
        
        request.parse(data);
        request
    }
    
    fn parse(&mut self, data: &[u8]) {
        let data_str = String::from_utf8_lossy(data);
        let lines: Vec<&str> = data_str.split("\r\n").collect();
        
        if !lines.is_empty() {
            let request_line = lines[0];
            let words: Vec<&str> = request_line.split(' ').collect();
            
            if !words.is_empty() {
                self.method = Some(words[0].to_string());
            }
            
            if words.len() > 1 {
                self.uri = Some(words[1].to_string());
            }
            
            if words.len() > 2 {
                self.http_version = words[2].to_string();
            }
        }
    }
}

struct HTTPHandler {
    headers: HashMap<String, String>,
    status_codes: HashMap<u16, String>,
    mime_types: HashMap<String, String>,
}

impl HTTPHandler {
    fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server".to_string(), "CrudeServer".to_string());
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        let mut status_codes = HashMap::new();
        status_codes.insert(200, "OK".to_string());
        status_codes.insert(404, "Not Found".to_string());
        status_codes.insert(501, "Not Implemented".to_string());
        
        let mut mime_types = HashMap::new();
        mime_types.insert("html".to_string(), "text/html".to_string());
        mime_types.insert("htm".to_string(), "text/html".to_string());
        mime_types.insert("css".to_string(), "text/css".to_string());
        mime_types.insert("js".to_string(), "application/javascript".to_string());
        mime_types.insert("jpg".to_string(), "image/jpeg".to_string());
        mime_types.insert("jpeg".to_string(), "image/jpeg".to_string());
        mime_types.insert("png".to_string(), "image/png".to_string());
        mime_types.insert("gif".to_string(), "image/gif".to_string());
        mime_types.insert("svg".to_string(), "image/svg+xml".to_string());
        mime_types.insert("ico".to_string(), "image/x-icon".to_string());
        
        HTTPHandler {
            headers,
            status_codes,
            mime_types,
        }
    }
    
    fn response_line(&self, status_code: u16) -> Vec<u8> {
        let reason = self.status_codes.get(&status_code).unwrap_or(&"Unknown".to_string());
        format!("HTTP/1.1 {} {}\r\n", status_code, reason).into_bytes()
    }
    
    fn response_headers(&self, extra_headers: Option<HashMap<String, String>>) -> Vec<u8> {
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
    
    fn get_mime_type(&self, filename: &str) -> String {
        if let Some(extension) = Path::new(filename).extension() {
            if let Some(ext_str) = extension.to_str() {
                if let Some(mime) = self.mime_types.get(ext_str) {
                    return mime.clone();
                }
            }
        }
        "text/html".to_string()
    }
    
    fn handle_GET(&self, request: &HTTPRequest) -> Vec<u8> {
        let uri = request.uri.as_ref().map_or("", |s| s.as_str());
        let filename = uri.trim_start_matches('/');
        
        if !filename.is_empty() && Path::new(filename).exists() {
            match File::open(filename) {
                Ok(mut file) => {
                    let mut response_body = Vec::new();
                    if file.read_to_end(&mut response_body).is_ok() {
                        let response_line = self.response_line(200);
                        
                        let content_type = self.get_mime_type(filename);
                        let mut extra_headers = HashMap::new();
                        extra_headers.insert("Content-Type".to_string(), content_type);
                        
                        let response_headers = self.response_headers(Some(extra_headers));
                        let blank_line = b"\r\n";
                        
                        let mut response = Vec::new();
                        response.extend_from_slice(&response_line);
                        response.extend_from_slice(&response_headers);
                        response.extend_from_slice(blank_line);
                        response.extend_from_slice(&response_body);
                        
                        return response;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to open file '{}': {}", filename, e);
                }
            }
        }
        
        let response_line = self.response_line(404);
        let response_headers = self.response_headers(None);
        let blank_line = b"\r\n";
        let response_body = b"<h1>404 Not Found</h1>";
        
        let mut response = Vec::new();
        response.extend_from_slice(&response_line);
        response.extend_from_slice(&response_headers);
        response.extend_from_slice(blank_line);
        response.extend_from_slice(response_body);
        
        response
    }
    
    fn HTTP_501_handler(&self, request: &HTTPRequest) -> Vec<u8> {
        let response_line = self.response_line(501);
        let response_headers = self.response_headers(None);
        let blank_line = b"\r\n";
        let response_body = b"<h1>501 Not Implemented</h1>";
        
        let mut response = Vec::new();
        response.extend_from_slice(&response_line);
        response.extend_from_slice(&response_headers);
        response.extend_from_slice(blank_line);
        response.extend_from_slice(response_body);
        
        response
    }
}

impl RequestHandler for HTTPHandler {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        let request = HTTPRequest::new(data);
        
        println!("Method: {:?}", request.method);
        println!("URI: {:?}", request.uri);
        println!("HTTP Version: {}", request.http_version);
        
        match &request.method {
            Some(method) if method == "GET" => self.handle_GET(&request),
            _ => self.HTTP_501_handler(&request),
        }
    }
}

fn main() -> std::io::Result<()> {
    let handler = HTTPHandler::new();
    let server = TCPServer::new("127.0.0.1".to_string(), 8888, handler);
    server.start()
}