use std::time::Duration;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn worker_task() -> io::Result<()> {
    loop {
        // Attempt to connect to the server
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(mut stream) => {
                // Wait for the server to send the worker ID
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).await?;
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from server: {}", message);

                // Extract the worker ID from the server's message
                // Assuming the server sends a message like "Your ID is 1"
                let id = message
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Failed to parse ID");

                // After receiving ID, worker can send its reporting message
                let report_message = format!("Worker {} reporting in", id);
                stream.write_all(report_message.as_bytes()).await?;

                // Continually listen for messages from the server
                loop {
                    match stream.read(&mut buffer).await {
                        Ok(n) if n == 0 => break, // Connection was closed
                        Ok(n) => {
                            println!(
                                "Worker {} received: {}",
                                id,
                                String::from_utf8_lossy(&buffer[..n])
                            );
                        }
                        Err(e) => {
                            eprintln!("An error occurred in worker {}: {}", id, e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to the server: {}", e);
            }
        }

        // Wait before attempting to reconnect
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    match worker_task().await {
        Ok(_) => println!("Worker completed its task"),
        Err(e) => eprintln!("Worker encountered an error: {}", e),
    };
}
