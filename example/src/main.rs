use std::io::{BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use xyo_sdk::enrichment::EnrichmentRequest;
// use xyo_sdk::client;
// use xyo_sdk::enrichment::{Enrichment, EnrichmentRequest};

fn main() {
    // const HOST: &str = "api.xyo.financial";
    // const PORT: i32 = 80;
    //
    // let Ok(mut tcpStreamSocket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
    //     todo!()
    // };
    //
    // // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    // tcpStreamSocket
    //     .set_read_timeout(Some(Duration::from_millis(100)))
    //     .unwrap();
    //
    // let _ = tcpStreamSocket.write(format!("GET /healthz HTTP/1.1\r\nHost: {}\r\n\r\n", HOST).as_bytes());
    // let a: &mut String = &mut String::new();
    // let _ = tcpStreamSocket.read_to_string(a);
    //
    // println!("Response from request:");
    // println!("{}", a);

    let resp = get("/healthz");
    println!("{}", resp);

    // socket.unwrap().write(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host).as_bytes()).expect("write");
    // socket.unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");

    // let client = client::new(client::ClientConfig {
    //     api_key: String::from("hello"),
    // });
    //
    // client.enrich_transaction(&EnrichmentRequest {
    //     content: String::from("Syniol AI Payment Enrichment Software"),
    //     country_code: String::from("GB"),
    // });

    println!("Successfully imported XYO SDK and created enrichment request");
}

const HOST: &str = "api.xyo.financial";
const PORT: i32 = 80;
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);

fn get(path: &str) -> String {
    let Ok(mut tcp_stream_socket) = TcpStream::connect(format!("{}:{}", HOST, PORT)) else {
        todo!()
    };

    // let addr = SocketAddr::from(([185, 185, 127, 12], 80));
    // let Ok(mut socket) = TcpStream::connect_timeout(&addr, Duration::from_millis(100))
    let _ = tcp_stream_socket
        .set_read_timeout(Some(DEFAULT_TIMEOUT));

    let _ = tcp_stream_socket.write(format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, HOST).as_bytes());
    let a: &mut String = &mut String::new();
    let _ = tcp_stream_socket.read_to_string(a);

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

    format!("{}", a)
}
