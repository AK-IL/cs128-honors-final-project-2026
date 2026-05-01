// Basic & common networking code
// Ignore the comments if they are too much, they are there to explain tokio specific code

use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::{Packet, Navigation};

pub struct Relay {
    address: String,
    clients: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<String>>>>,
    rooms: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    global_tx: broadcast::Sender<String>,
}

impl Relay {
    pub fn new(addr: &str, buffer: usize) -> Self {
        let (global_tx, _) = broadcast::channel(buffer);
        Self {
            address: addr.to_string(),
            clients: Arc::new(RwLock::new(HashMap::new())),
            rooms: Arc::new(RwLock::new(HashMap::new())),
            global_tx,
        }
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Return type based on https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
        // Using tokio for async since synchronous networking would block threads and break with many users

        let tcp_listener = TcpListener::bind(&self.address).await?; // Change port address in production
        
        // [Good place to log server starting at address]

        loop {
            let (mut socket, _addr) = tcp_listener.accept().await?; // Wait for new connection
            let clients = Arc::clone(&self.clients);
            let rooms = Arc::clone(&self.rooms);
            let global_tx = self.global_tx.clone(); // Clones transmitter so that ownership doesn't conflict with multiple users
            let mut global_rx = global_tx.subscribe(); // Don't forget to smash that like button and hit the bell so you never miss another video!

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split(); // Splits socket into read/write halves
                let mut reader = BufReader::new(reader);
                let (private_tx, mut private_rx) = mpsc::unbounded_channel::<String>();
                let mut current_user_id = String::new();
                let mut line = String::new();
                loop {
                    tokio::select! { // Allows for running the two processes concurrently
                        read = reader.read_line(&mut line) => {
                            if read.unwrap_or(0) == 0 { break; }
                            if let Ok(packet) = serde_json::from_str::<Packet>(&line) {
                                // Update or set UID
                                if current_user_id.is_empty() {
                                    current_user_id = packet.source.clone();
                                    clients.write().unwrap().insert(current_user_id.clone(), private_tx.clone());
                                }
                                match &packet.navi {
                                    Navigation::Everyone => {
                                        // Send to the broadcast channel
                                        let _ = global_tx.send(line.clone());
                                    },
                                    Navigation::User(target) => {
                                        // Direct Routing
                                        let register = clients.read().unwrap();
                                        if let Some(target_tx) = register.get(target) {
                                            // Send only to the specific user/room pipe
                                            let _ = target_tx.send(line.clone());
                                        } else {
                                            // [Optional: Send error back to user - Target Not Found]
                                        }
                                    },
                                    Navigation::Room(room) => {
                                        // Join room
                                        {
                                            let mut rooms_guard = rooms.write().unwrap();
                                            rooms_guard.entry(room.clone()).or_insert_with(HashSet::new).insert(current_user_id.clone());
                                        }
                                        // Send to room
                                        let rooms_guard = rooms.read().unwrap();
                                        if let Some(members) = rooms_guard.get(room) {
                                            let clients_guard = clients.read().unwrap();
                                            for id in members {
                                                if id != &current_user_id {
                                                    if let Some(tx) = clients_guard.get(id) {
                                                        let _ = tx.send(line.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            line.clear();
                        }
                        // Targeted recieved
                        Some(msg) = private_rx.recv() => {
                            if writer.write_all(msg.as_bytes()).await.is_err() { break; }
                        }

                        // Global recieved
                        Ok(msg) = global_rx.recv() => {
                            if writer.write_all(msg.as_bytes()).await.is_err() { break; }
                        }
                    }
                }
                // Cleanup
                if !current_user_id.is_empty() {
                    clients.write().unwrap().remove(&current_user_id);
                    // Remove from rooms
                    let mut rooms_guard = rooms.write().unwrap();
                    for members in rooms_guard.values_mut() {
                        members.remove(&current_user_id);
                    }
                    // Remove keys from set
                    rooms_guard.retain(|_, members| !members.is_empty());
                }
                
            });

            // [Good place to log client disconnecting]
        }

        // Ok(()) <-- Use if compiler complains about return type
        }
}