mod err;

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
pub use crate::err::HttpClientError;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

const HOST: &str = "api.xyo.financial";
const PORT: i32 = 443;
const DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: i16,
    pub body: String,
}

mod http_message {
    use crate::{HttpMethod, HOST};

    /// Constructs the RFC Standard Header for HTTP 1.1 Specs, safely parsing headers and body length.
    pub fn new(method: HttpMethod, path: &str, headers: &[(&str, &str)], data: &str) -> String {
        let method_str = match method {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
        };

        // HTTP/1.1 requires Host header and we enforce Connection: close for synchronous non-keep-alive sockets
        let mut req = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n",
            method_str, path, HOST
        );

        // Inject dynamic headers (like Authorization)
        for (k, v) in headers {
            req.push_str(&format!("{}: {}\r\n", k, v));
        }

        // Apply defaults if not provided in headers
        if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Content-Type")) && !data.is_empty() {
            req.push_str("Content-Type: application/json\r\n");
        }
        if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Accept")) {
            req.push_str("Accept: application/json\r\n");
        }

        if !data.is_empty() {
            req.push_str(&format!("Content-Length: {}\r\n", data.as_bytes().len()));
        }

        // End of headers
        req.push_str("\r\n");
        
        // Append body
        if !data.is_empty() {
            req.push_str(data);
        }

        req
    }
}

/// Send an HTTP request to the API, now securely via TLS and supporting header injection.
pub fn request(
    method: HttpMethod,
    path: &str,
    headers: &[(&str, &str)],
    data: &str,
) -> Result<HttpResponse, HttpClientError> {
    let tcp_stream_socket = TcpStream::connect(format!("{}:{}", HOST, PORT)).map_err(|e| {
        HttpClientError {
            code: 503,
            message: format!("could not connect to host: {} and port number {}: {}", HOST, PORT, e),
        }
    })?;

    tcp_stream_socket
        .set_read_timeout(Some(DEFAULT_TIMEOUT_DURATION))
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed to set read timeout: {}", e),
        })?;

    let connector = TlsConnector::new().map_err(|e| HttpClientError {
        code: 500,
        message: format!("failed to create TLS connector: {}", e),
    })?;

    let mut secure_stream = connector
        .connect(HOST, tcp_stream_socket)
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed TLS handshake with {}: {}", HOST, e),
        })?;

    let req_payload = http_message::new(method, path, headers, data);
    
    secure_stream
        .write_all(req_payload.as_bytes())
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed to write to socket: {}", e),
        })?;

    // Read the entire response (headers + body) until EOF (which is safe because Connection: close)
    let mut resp_bytes = Vec::new();
    secure_stream
        .read_to_end(&mut resp_bytes)
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed to read from socket: {}", e),
        })?;

    let _ = secure_stream.flush();

    // Parse HTTP response safely without assuming \r\n separated JSON body
    let response_str = String::from_utf8_lossy(&resp_bytes);
    let mut parts = response_str.splitn(2, "\r\n\r\n");
    let header_part = parts.next().unwrap_or("");
    let body_part = parts.next().unwrap_or("").to_string();

    let status_line = header_part.lines().next().unwrap_or("");
    let status_parts: Vec<&str> = status_line.splitn(3, ' ').collect();
    let status_code: i16 = if status_parts.len() >= 2 {
        status_parts[1].parse().unwrap_or(500)
    } else {
        500
    };

    Ok(HttpResponse {
        status_code,
        body: body_part,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_without_body_content() {
        // Just a simple ping to see if TLS + Parsing works. Healthz might not be an endpoint, so it might 404, but it shouldn't error locally
        let result = request(HttpMethod::GET, "/healthz", &[], "");
        assert!(result.is_ok());
    }
}
