use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    // Send player name
    let name = std::env::args().nth(1).unwrap_or_else(|| "Player".to_string());
    stream.write_all(name.as_bytes()).await?;

    let mut buffer = [0; 1024];

    // Read welcome message
    let n = stream.read(&mut buffer).await?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    // Simple loop to send messages and receive responses
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if input.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        stream.write_all(input.as_bytes()).await?;

        let n = stream.read(&mut buffer).await?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}