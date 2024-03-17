use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener};

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
            println!("{}", message);
            stream.write_all(&buf[..n])?;
        }
    }

    Ok(())
}