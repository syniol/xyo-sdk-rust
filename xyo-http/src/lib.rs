use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

const HOST: &str = "api.xyo.financial";
const PORT: i32 = 80;
const DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_millis(50);

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
            )
        }

        format!("{:?} {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\n\n",method, path, HOST)
    }
}


/// It will send an HTTP request to XYO API
/// method: HttpMethod only accepts POST and GET at the moment
/// path: Starts with `/` e.g. /api/v1/enrichment
/// data: Body is bytes
pub fn request(method: HttpMethod, path: &str, data: &str) -> String {
    let Ok(mut tcp_stream_socket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
        todo!()
    };

    // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    let _ = tcp_stream_socket.set_read_timeout(Some(DEFAULT_TIMEOUT_DURATION));
    let _ = tcp_stream_socket.write(http_message::new(method, path, data).as_bytes());
    let a: &mut String = &mut String::new();
    let _ = tcp_stream_socket.read_to_string(a);
    tcp_stream_socket.flush().unwrap();
    tcp_stream_socket.shutdown(Shutdown::Both).unwrap();

    format!("{}", a)
}

pub fn get_body_from_request_response(result: String) -> String {
    let response_vector = result.split("\r\n").collect::<Vec<&str>>();

    String::from(remove_whitespace(response_vector[response_vector.len() - 1]))
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_without_body_content() {
        let result = request(HttpMethod::GET, "/healthz", "");
        assert_eq!(result.contains("\"healthy\": true"), true);
    }

    #[test]
    fn it_works_with_body_content() {
        let result = request(HttpMethod::GET, "/healthz", "{\"status\": \"something\"}");
        let response_body = get_body_from_request_response(result);

        #[derive(Serialize, Deserialize, Debug)]
        struct HealthCheck {
            healthy: bool,
        }
        let p: HealthCheck = serde_json::from_str(response_body.as_str()).expect("REASON");

        println!("{}", response_body);
        assert_eq!(p.healthy, true);
        assert_eq!(response_body.contains("\"healthy\":true"), true);
    }
}
