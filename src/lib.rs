pub mod key_exchange;
pub mod crypto;

use serde::{Serialize, Deserialize};

// Protocol so client & server are in agreement
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Navigation {
    Everyone, // Send to everyone
    Room(String), // Send to group
    User(String), // Send to person
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    KeyExchange([u8; 32]), // Diffie-Hellman key exchnage
    Text(Vec<u8>), // Encrypted plaintext message (including the nonce)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    pub navi: Navigation, // Destination
    pub timestamp: u64, // Timestamp
    pub source: String, // Sender
    pub data: Data, // Message content
}