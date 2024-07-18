use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::env;
use std::str;

fn handle_connection(mut stream: TcpStream) {
    let peer_addr: std::net::SocketAddr = match stream.peer_addr() {
        Ok(peer) => peer,
        Err(e) => {
            println!("Error handling connection: {e}");
            return;
        }
    };
    println!("Accepted connection from {peer_addr}", );

    let mut buffer: [u8; 1024] = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => bytes_read,
        Err(err) => {
            println!("Failed to read from stream: {err}");
            return;
        },
    };

    let data: &str = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e);
            match stream.shutdown(Shutdown::Both) {
                Ok(_) => return,
                Err(_) => {
                    println!("Failed to shutdown stream");
                    return;
                }
            }
        },
    };

    println!("Received data: {data}");

    let _ = stream.write(b"Hello World");
    let _ = stream.flush();

    match stream.shutdown(Shutdown::Both) {
        Ok(_) => return,
        Err(_) => {
            println!("Failed to shutdown stream");
            return;
        }
    }
}

fn main() {
    let port: String = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8000"),
    };
    let listener:TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Error opening tcp listener");
    println!("Listening on port: {port}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("Error accepting connection: {e}")
        }
    }
}
