use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;


const HOST: &str = "api.xyo.financial";
const PORT: i32 = 80;
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(50);

// GET / HTTP/1.1
// Host: example.com
// User-Agent: curl/8.2.1
// Accept: */*
fn get(path: &str) -> String {
    let Ok(mut tcp_stream_socket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
        todo!()
    };

    // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    let _ = tcp_stream_socket.set_read_timeout(Some(DEFAULT_TIMEOUT));
    let _ = tcp_stream_socket.write(format!("GET {} HTTP/1.1\nHost: {}\n\n", path, HOST).as_bytes());
    let a: &mut String = &mut String::new();
    let _ = tcp_stream_socket.read_to_string(a);
    tcp_stream_socket.flush().unwrap();
    tcp_stream_socket.shutdown(Shutdown::Both).unwrap();

    format!("{}", a)
}

fn post(path: &str, data: &str) -> String {
    let Ok(mut tcp_stream_socket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
        todo!()
    };

    // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    tcp_stream_socket
        .set_read_timeout(Some(DEFAULT_TIMEOUT))
        .unwrap();

    let _ = tcp_stream_socket.write(format!("POST {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, HOST).as_bytes());
    let a: &mut String = &mut String::new();
    let _ = tcp_stream_socket.read_to_string(a);
    // tcp_stream_socket.flush().unwrap();
    // tcp_stream_socket.shutdown(Shutdown::Both).unwrap();

    format!("{}", a)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
