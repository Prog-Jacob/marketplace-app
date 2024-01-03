use std::net::SocketAddr;
use utils::load_env;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();
    let addr: SocketAddr = format!("[::1]:{}", std::env::var("SERVER_PORT")?)
        .parse()
        .unwrap();
    println!("Server is listening on http://{}", addr);
    Ok(())
}
