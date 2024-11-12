use std::io::{self, Write};

fn main() {
    let response = handle_request();
    io::stdout().write_all(response.as_bytes()).unwrap();
}

fn handle_request() -> String {
    // Simulate handling a request and returning a response
    let response_body = "Hello from Rust on Vercel!";
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}