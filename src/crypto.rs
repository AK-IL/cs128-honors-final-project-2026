use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use rand::{rngs::OsRng, RngCore};

pub fn encrypt_txt(key: &[u8; 32], txt: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Key length invalid");
    let mut nb = [0u8; 12];
    OsRng.fill_bytes(&mut nb);
    let nonce = Nonce::from_slice(&nb);
    let mut encrypted = cipher.encrypt(nonce, txt.as_bytes()).expect("Failed to encrypt");
    let mut packet = nb.to_vec();
    packet.append(&mut encrypted);
    packet
}

pub fn decrypt_txt(key: &[u8; 32], mut msg: Vec<u8>) -> Option<String> {
    if msg.len() < 12 {
        return None;
    }
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key");
    let nb = msg.drain(..12).collect::<Vec<u8>>();
    let nonce = Nonce::from_slice(&nb);
    let tb = cipher.decrypt(nonce, msg.as_ref()).ok()?;
    String::from_utf8(tb).ok()
}