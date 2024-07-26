use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::error::Error;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt};

use crate::request;

pub struct Server {
    pub port: String,
}

impl Server {
    pub async fn start(&self) {
        let listener: TcpListener =
            TcpListener::bind(format!("127.0.0.1:{}", self.port)).await.expect("Error opening tcp listener");
        println!(
            "🚀 listening on address: http://{}",
            listener.local_addr().unwrap()
        );

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => match Self::handle_connection(stream, addr).await {
                    Ok(_) => tracing::info!("Connection handled successfully"),
                    Err(err) => tracing::error!("Connection handling failed with error: {err}"),
                },
                Err(e) => tracing::error!("Error accepting connection: {e}"),
            }
        }
    }

    async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        tracing::info!("Accepted connection from {addr}",);
    
        let mut reader: BufReader<TcpStream> = BufReader::new(stream);
        let data: Vec<String> = Self::read_http_request(&mut reader).await?;
        let request = request::Request::build_request_body(data);
        tracing::debug!("Received request: {:#?}", request);

        let mut stream = reader.into_inner();
    
        stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello World\r\n").await?;
        stream.flush().await?;
        stream.shutdown().await?;
        Ok(())
    }

    async fn read_http_request(reader: &mut BufReader<TcpStream>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut headers: String = String::new();
    
        // Read headers until we encounter "\r\n\r\n"
        loop {
            let bytes_read = reader.read_line(&mut headers).await?;
            if bytes_read < 3 {
                break;
            }
        }
    
        let mut body: Vec<&str> = headers.lines().collect();
        let mut content_length: usize = 0;
        for header in &body {
            let l_case_header: String = header.to_lowercase();
            if l_case_header.starts_with("content-length") {
                let content_length_header: Vec<&str> =
                    header.split(":").map(|l: &str| l.trim()).collect();
                content_length = content_length_header[1].parse().unwrap_or(0);
            }
        }
    
        let mut body_buffer: Vec<u8> = vec![0; content_length];
        reader.read_exact(&mut body_buffer).await?;
    
        let data: String = String::from_utf8(body_buffer)?;
        body.push(&data);
        let request: Vec<String> = body.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    
        Ok(request)
    }
}
