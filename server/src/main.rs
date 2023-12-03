use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    // Shared counter for worker IDs
    let worker_id_counter = Arc::new(AtomicUsize::new(1));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let counter = worker_id_counter.clone();

        tokio::spawn(async move {
            // Assign an ID to the worker
            let worker_id = counter.fetch_add(1, Ordering::SeqCst);

            // Send the assigned ID back to the worker
            let id_message = format!("Your ID is {}", worker_id);
            if let Err(e) = socket.write_all(id_message.as_bytes()).await {
                eprintln!("Failed to write to socket; err = {:?}", e);
                return;
            }

            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        // Process the message here
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        println!("Server received from worker {}: {}", worker_id, message);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
