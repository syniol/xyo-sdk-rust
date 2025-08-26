use std::io::{BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
// use xyo_sdk::client;
// use xyo_sdk::enrichment::{Enrichment, EnrichmentRequest};

fn main() {
    let host = "api.xyo.financial";
    let addr = SocketAddr::from(([185, 185, 127, 12], 80));

    let  Ok(mut socket) = TcpStream::connect(addr) else { todo!() };
    socket.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
    println!("Connected to {}", addr.ip());
    let _ = socket.write(format!("GET /healthz HTTP/1.1\r\nHost: {}\r\n\r\n", host).as_bytes());

    let a: &mut String = &mut String::new();
    socket.read_to_string(a);
    println!("{}", a);


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
