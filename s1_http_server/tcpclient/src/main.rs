use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    tcp_stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    tcp_stream.read(&mut buffer).unwrap();
    println!(
        "response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
