use std::env;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

fn read_http_request(stream: &TcpStream) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut reader: BufReader<&TcpStream> = BufReader::new(stream);

    let mut headers: String = String::new();

    // Read headers until we encounter "\r\n\r\n"
    loop {
        let bytes_read = reader.read_line(&mut headers)?;
        if bytes_read < 3 {
            break
        }
    }

    let mut headers_array: Vec<&str> = headers.lines().collect();
    let mut content_length: usize = 0;
    for header in &headers_array {
        let l_case_header: String = header.to_lowercase();
        if l_case_header.starts_with("content-length") {
            let content_length_header: Vec<&str> = header.split(":").map(|l: &str| l.trim()).collect();
            content_length = content_length_header[1].parse().unwrap_or(0);
        }
    }

    let mut body_buffer: Vec<u8> = vec![0; content_length];
    reader.read_exact(&mut body_buffer)?;

    let data: String =  String::from_utf8(body_buffer)?;
    headers_array.push(&data);
    let request: Vec<String> = headers_array.iter().map(|&s| s.to_string()).collect::<Vec<String>>();

    Ok(request)
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer_addr: SocketAddr = stream.peer_addr()?;

    println!("Accepted connection from {peer_addr}",);

    let request: Vec<String> = read_http_request(&stream)?;
    println!("Received request: {:?}", request);

    stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello World\r\n")?;
    stream.flush()?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() {
    let port: String = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8000"),
    };
    let listener: TcpListener =
        TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Error opening tcp listener");
    println!("🚀 listening on port: {port}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => println!("Connection handled successfully"),
                Err(err) => println!("Connection handling failed with error: {err}"),
            },
            Err(e) => println!("Error accepting connection: {e}"),
        }
    }
}
