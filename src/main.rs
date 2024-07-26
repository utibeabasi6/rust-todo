mod request;
mod http;

use std::env;


#[tokio::main]
async fn main() {
    let port: String = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8000"),
    };

    let server = http::Server{port};
    server.start().await;
    
}
