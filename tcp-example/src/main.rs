use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let conn = TcpListener::bind("127.0.0.1:8080").expect("Error establishing connection");
    println!("Running on PORT 8080");
    for stream in conn.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buf = [0; 100];
        stream.read(&mut buf).expect("Error reading stream");
        println!("{}", String::from_utf8_lossy(&buf));
        stream.write(&mut buf).unwrap();
    }
}
