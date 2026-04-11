// Basic & common networking code
// Ignore the comments if they are too much, they are there to explain tokio specific code

use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::Packet;

pub struct Relay {
    address: String,
    transmitter: broadcast::Sender<String>,
}

impl Relay {
    pub fn new(addr: &str, buffer: usize) -> Self {
        let (transmitter, _) = broadcast::channel(buffer);
        Self {
            address: addr.to_string(),
            transmitter,
        }
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Return type based on https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
        // Using tokio for async since synchronous networking would block threads and break with many users

        let tcp_listener = TcpListener::bind(&self.address).await?; // Change port address in production
        
        // [Good place to log server starting at address]

        loop {
            let (mut socket, _addr) = tcp_listener.accept().await?; // Wait for new connection
            let transmitter = self.transmitter.clone(); // Clones transmitter so that ownership doesn't conflict with multiple users
            let mut reciever = transmitter.subscribe(); // Don't forget to smash that like button and hit the bell so you never miss another video!

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split(); // Splits socket into read/write halves
                let mut reader = BufReader::new(reader);
                let mut line = String::new();
                loop {
                    tokio::select! { // Allows for running the two processes concurrently
                        read = reader.read_line(&mut line) => { // Proc 1: Read from bound client
                            if read.unwrap_or(0) == 0 {
                                break;
                            } // Exit if client gets disconnected
                            if serde_json::from_str::<Packet>(&line).is_ok() {
                                let _ = transmitter.send(line.clone()); // Send JSON to everyone after checking for validity
                            }
                            line.clear();
                        }
                        read = reciever.recv() => { // Proc 2: Listen for msgs from other clients
                            if let Ok(msg) = read {
                                if writer.write_all(msg.as_bytes()).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            });

            // [Good place to log client disconnecting]
        }

        // Ok(()) <-- Use if compiler complains about return type
        }
}