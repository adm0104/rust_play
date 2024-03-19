use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::fs;

fn main() -> io::Result<()> {
    // Create a TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on: {}", addr);

    // Accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("New connection: {}", stream.peer_addr()?);

        let mut buf = [0; 1024];
        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let message = String::from_utf8_lossy(&buf[..n]);
            println!("Searching for file: {}", message);
            let file_found_message: String = search_file_path(message.to_string());
            stream.write_all(file_found_message.as_bytes())?;

        }
    }

    Ok(())
}

fn search_file_path(path: String) -> String {
    match fs::metadata(path) {
        Ok(_path) => {
            return "File found".to_string();
        }
        Err(_e) => {
            return "File not found".to_string();
        }
    }
}