// src/main.rs
mod game;
mod network;

use network::server::GameServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = GameServer::new();
    server.run("127.0.0.1:8080").await?;
    Ok(())
}