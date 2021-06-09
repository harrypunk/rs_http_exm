use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

const DEST: &str = "httpbin.org:80";

fn main() {
    match TcpStream::connect(DEST) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 80");

            let msg: &str = "GET /ip HTTP/1.1\r\nHost:123\r\n\r\n";

            stream.write(msg.as_bytes()).unwrap();

            let mut data = [0 as u8; 1024];
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Response: {}", text);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
