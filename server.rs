#![allow(non_snake_case)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {
    openWebServer(3030)
}

fn openWebServer(port: u32) {
    let addressString = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addressString).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handleConnection(_stream);
    }
}

fn handleConnection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let lines = buf_reader.lines();
    // println!("BufReader lines:\n{:#?}", lines);
    // let mapped = lines.map(|result| result.unwrap());
    // println!("Mapped:\n{:#?}", mapped);
    // let tw = mapped.take_while(|line| !line.is_empty());
    // println!("Take While\n:{:#?}", tw);
    // let coll: Vec<_> = tw.collect();
    // println!("collect\n:{:#?}", coll);
    // println!("BufReader lines:\n{:#?}", buf_reader.lines());
    
    // Why is a take_while required here? From my reading it's just a ternary... What's it returning that collect requires?
    // let lines = buf_reader.lines();
    // let mapped = lines.map(|r| r.unwrap());
    // let tw = mapped.take_while(|line| !line.is_empty());
    // let coll: Vec<_> = tw.collect();
    // println!("{:#?}", coll)
    
    // let lines: Vec<_> = buf_reader.lines().collect::<Result<_, _>>().unwrap();
    // println!("{:#?}", lines);
    
    
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //      // For each result, result = result.unwrap() (so _result)
    //      // Just converts it to an iterator for the lines?
    //     .map(|result| result.unwrap())
    //      // Takes the above iterator and only passes it on when the line isn't empty
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?}", http_request);
}