mod request;
mod http;

use std::env;


#[tokio::main]
async fn main() {
    let port: String = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8000"),
    };

    let subscriber = tracing_subscriber::fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_thread_ids(false)
    .with_target(false)
    .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to subscribe to traces");

    let server = http::Server{port};
    server.start().await;
    
}
