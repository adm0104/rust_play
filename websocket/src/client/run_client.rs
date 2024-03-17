use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:8081";
    let mut stream = TcpStream::connect(server_addr)?;
    println!("Connected to server: {}", server_addr);

    let message = "Hello, server!";
    stream.write_all(message.as_bytes())?;
    println!("Sent message: {}", message);

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received response: {}", response);

    Ok(())
}