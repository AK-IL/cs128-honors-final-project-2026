// Basic & common networking code

use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use ata_messenger::{Packet, Navigation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Return type based on https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
    // Using tokio for async since synchronous networking would block threads and break with many users

    let address = "127.0.0.1:8080";
    let tcp_listener = TcpListener::bind(address).await?; // Change port address in production
    let (transmitter, _) = broadcast::channel::<String>(32); // 32 message buffer
    
    // [Good place to log server starting at address]

    loop {
        let (mut socket, _addr) = tcp_listener.accept().await?; // Wait for new connection
        let transmitter = transmitter.clone(); // Clones transmitter so that ownership doesn't conflict with multiple users
        let mut reciever = transmitter.subscribe(); // Don't forget to smash that like button and hit the bell so you never miss another video!

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split(); // Splits socket into read/write halves
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! { // Allows for running the two processes concurrently
                    read = reader.read_line(&mut line) => { // Proc 1: Read from bound client
                        if read.unwrap_or(0) == 0 { break; } // Exit if client gets disconnected
                        if let Ok(_packet) = serde_json::from_str::<Packet>(&line) {
                            let _ = transmitter.send(line.clone()); // Send JSON to everyone after checking for validity
                        }
                        line.clear();
                    }
                    read = reciever.recv() => { // Proc 2: Listen for msgs from other clients
                        if let Ok(msg) = read {
                            let _ = writer.write_all(msg.as_bytes()).await; // Send over TCP
                        }
                    }
                }
            }
        });

        // [Good place to log client disconnecting]
    }

    // Ok(()) <-- Use if compiler complains about return type
}