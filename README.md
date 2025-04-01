# Rust HTTP Server

A simple HTTP server built from scratch in Rust. This project demonstrates how to create a basic web server without using any HTTP-specific libraries, implementing the HTTP protocol directly over TCP sockets.

## Features

- Listens for HTTP requests on a specified port (default: 8888)
- Parses incoming HTTP requests
- Serves static files with appropriate MIME types
- Handles GET requests
- Returns proper status codes (200 OK, 404 Not Found, 501 Not Implemented)
- Supports serving HTML, images, and other media types

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system
- Basic understanding of HTTP protocol
- Basic knowledge of Rust programming

### Installation

1. Clone this repository:
   ```
   git clone https://github.com/NithinKonda/httpserver.git
   cd rust-http-server
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the server:
   ```
   cargo run --release
   ```

## Usage

Once the server is running, it listens on `127.0.0.1:8888` by default. You can place HTML files, images, and other content in the same directory as the executable.

To access your files:
- Open a web browser and navigate to `http://127.0.0.1:8888/filename.html`
- Images can be accessed directly at `http://127.0.0.1:8888/image.png` or embedded in HTML files

### Example Files

Create these files in the same directory as your executable to test the server:

**index.html**:
```html
<html>
    <head>
        <title>Index page</title>
    </head>
    <body>
        <h1>Index page</h1>
        <p>This is the index page.</p>
        <img src="example.png" alt="Example image">
    </body>
</html>
```

**hello.html**:
```html
<html>
    <head>
        <title>Hello page</title>
    </head>
    <body>
        <h1>Hello page</h1>
        <p>This is the hello page.</p>
    </body>
</html>
```

Also add some image files (PNG, JPG, etc.) to test the media serving capabilities.

## Project Structure

- `TCPServer`: Generic TCP server that accepts connections and passes data to a handler
- `RequestHandler`: Trait that defines how requests are handled
- `HTTPRequest`: Parses incoming HTTP requests into structured data
- `HTTPHandler`: Implements the `RequestHandler` trait for HTTP protocol
  - Handles GET requests
  - Serves files based on URI
  - Detects MIME types
  - Generates appropriate HTTP responses

## Customization

- To change the host or port, modify the parameters in the `main()` function
- Add support for additional MIME types by updating the `mime_types` HashMap
- Implement handlers for other HTTP methods (POST, PUT, etc.) by adding new methods to `HTTPHandler`

## Learning Objectives

This project demonstrates:
- Working with TCP sockets in Rust
- Implementing a protocol (HTTP) from scratch
- File I/O operations in Rust
- Error handling patterns
- Trait-based polymorphism

## Limitations

This is a basic HTTP server intended for learning purposes:
- No support for concurrent connections
- Limited error handling
- No virtual hosting
- No support for CGI or server-side processing
- No security features

## License

This project is open source and available under the MIT License.

## Acknowledgments

This project was inspired by a similar HTTP server tutorial for Python, adapted to Rust's programming paradigms and features.