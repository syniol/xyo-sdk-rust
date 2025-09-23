mod err;

use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;
pub use crate::err::HttpClientError;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

const HOST: &str = "api.xyo.financial";
const PORT: i32 = 80;
const DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_millis(100);

mod http_message {
    use crate::{HttpMethod, HOST};

    /// Constructs the RFC Standard Header for HTTP 1.1 Specs
    pub fn new(method: HttpMethod, path: &str, data: &str) -> String {
        if data.len() > 0 {
            return format!(
                "{:?} {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\n{}",
                method,
                path,
                HOST,
                data.len(),
                data,
            );
        }

        format!(
            "{:?} {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\n\n",
            method, path, HOST
        )
    }
}

/// It will send an HTTP request to XYO API
/// method: HttpMethod only accepts POST and GET at the moment
/// path: Starts with `/` e.g. /api/v1/enrichment
/// data: Body is string literal e.g. `"{\"key\":\"value\"}"`
pub fn request(method: HttpMethod, path: &str, data: &str) -> Result<String, HttpClientError> {
    let Ok(mut tcp_stream_socket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
        return Err(HttpClientError{
            code: 503,
            message: format!("could not connect to host: {} and port number {}", HOST, PORT),
        })
    };

    // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    let _ = tcp_stream_socket.set_read_timeout(Some(DEFAULT_TIMEOUT_DURATION));
    let _ = tcp_stream_socket.write(http_message::new(method, path, data).as_bytes());
    let resp: &mut String = &mut String::new();
    let _ = tcp_stream_socket.read_to_string(resp);
    tcp_stream_socket.flush().unwrap();
    tcp_stream_socket.shutdown(Shutdown::Both).unwrap();

    Ok(format!("{}", resp))
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
