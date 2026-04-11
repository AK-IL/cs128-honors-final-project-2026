pub mod key_exchange;
pub mod crypto;

use serde::{Serialize, Deserialize};

// Protocol so client & server are in agreement
#[derive(Serialize, Deserialize, Debug)]
pub enum MsgProtocol {
    DiffieHellman([u8; 32]), // Diffie-Hellman public key exchange bytes
    Etxt(Vec<u8>), // Encrypted txt (w/ nonce)s
}