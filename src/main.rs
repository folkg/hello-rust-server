use std::{net::TcpListener, println};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // one stream represents one connection
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
