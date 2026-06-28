mod err;

use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use native_tls::TlsConnector;
pub use crate::err::HttpClientError;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

const HOST: &str = "api.xyo.financial";
const PORT: i32 = 443;
const DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

mod http_message {
    use crate::{HttpMethod, HOST};

    /// Constructs the RFC Standard Header for HTTP 1.1 Specs
    pub fn new(method: HttpMethod, path: &str, data: &str) -> String {
        if data.len() > 0 {
            return format!(
                "{:?} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: application/json\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                method,
                path,
                HOST,
                data.len(),
                data,
            );
        }

        format!(
            "{:?} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: application/json\r\n\r\n",
            method, path, HOST
        )
    }
}

/// It will send an HTTP request to XYO API
/// method: HttpMethod only accepts POST and GET at the moment
/// path: Starts with `/` e.g. /api/v1/enrichment
/// data: Body is string literal e.g. `"{\"key\":\"value\"}"`
pub fn request(method: HttpMethod, path: &str, data: &str) -> Result<String, HttpClientError> {
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

    let mut secure_stream = connector.connect(HOST, tcp_stream_socket).map_err(|e| HttpClientError {
        code: 500,
        message: format!("failed TLS handshake with {}: {}", HOST, e),
    })?;

    secure_stream
        .write_all(http_message::new(method, path, data).as_bytes())
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed to write to socket: {}", e),
        })?;

    let mut resp = String::new();
    secure_stream
        .read_to_string(&mut resp)
        .map_err(|e| HttpClientError {
            code: 500,
            message: format!("failed to read from socket: {}", e),
        })?;

    let _ = secure_stream.flush();
    let _ = secure_stream.shutdown();

    Ok(resp)
}

/// It will get the last line of response with split after: \r\n
pub fn get_body_from_request_response(result: String) -> String {
    let response_vector = result.split("\r\n").collect::<Vec<&str>>();

    String::from(
        response_vector[response_vector.len() - 1],
    )
}

/// It will get the first line of response header: HTTP 200 OK and splits by space
/// Final output is an integer 16 byte size
pub fn get_status_code(result: String) -> i16 {
    let response_vector = result.split("\r\n").collect::<Vec<&str>>();
    let status_code_str = response_vector[0].split(" ").collect::<Vec<&str>>()[1];
    let status_code: i16 = status_code_str.trim().parse().unwrap();

    status_code
}

/// Hold-off
fn _remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_without_body_content() {
        let result = request(HttpMethod::GET, "/healthz", "");
        assert_eq!(result.is_err(), false);
        assert_eq!(result.ok().unwrap().contains("\"healthy\":true"), true);
    }

    #[test]
    fn it_works_with_body_content() {
        let resp = request(
            HttpMethod::GET,
            "/healthz",
            "{\"status\":\"something\"}",
        );

        let actual = resp.unwrap();

        let response_body = get_body_from_request_response(actual.clone());
        let status_code = get_status_code(actual.clone());

        println!("status_code: {}", status_code);

        assert_eq!(status_code, 200);
        assert_eq!(response_body.contains("\"healthy\":true"), true);
    }
}
