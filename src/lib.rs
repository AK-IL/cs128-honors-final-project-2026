pub mod key_exchange;
pub mod crypto;

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
    GroupKey(Vec<u8>), // Share encrypted group key
    Text(Vec<u8>), // Encrypted plaintext message (including the nonce)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    pub navi: Navigation, // Destination
    pub timestamp: u64, // Timestamp
    pub source: String, // Sender
    pub data: Data, // Message content
}

impl Packet {
    pub fn new(source: String, navi: Navigation, data: Data) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Clock is incorrect").as_secs();
        Self {
            source,
            navi,
            timestamp,
            data,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let mut json = serde_json::to_string(self)?;
        json.push('\n');
        Ok(json)
    }
}