#![allow(non_snake_case)]
use std::net::TcpListener;

fn main() {
    openWebServer(3030)
}

fn openWebServer(port: u32) {
    let addressString = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addressString).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Handled the connection");
    }
}