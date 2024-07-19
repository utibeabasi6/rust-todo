use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::env;

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer_addr: SocketAddr = stream.peer_addr()?;
    
    println!("Accepted connection from {peer_addr}", );

    let reader = BufReader::new(&stream);
    let request: Vec<String> = reader.lines().map(|l: Result<String, std::io::Error>| match l {
        Ok(line) => line,
        Err(_) => "".to_string(),
    }).take_while(|x: &String| !x.is_empty()).collect();
    
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
    let listener:TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Error opening tcp listener");
    println!("Listening on port: {port}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match handle_connection(stream) {
                    Ok(_) => println!("Connection handled successfully"),
                    Err(err) => println!("Connection handling failed with error: {err}")
                }
            }
            Err(e) => println!("Error accepting connection: {e}")
        }
    }
}
