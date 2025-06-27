use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("tcp listening...");
    for stream in listener.incoming() {
        // 思考:为什么这里stream要可变？
        // 因为下文读取和写回方法会改变其缓冲区
        let mut cur_stream = stream.unwrap();
        println!("tcp established");
        
        let mut buffer = [0; 1024];
        cur_stream.read(&mut buffer).unwrap();
        cur_stream.write(&mut buffer).unwrap();
    }
}
