use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) {
    // Read from stream
    let mut buffer = [0; 512];
    for _ in 0..3 {
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        stream.write(&buffer).unwrap();
    }
}
