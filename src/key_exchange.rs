// Note: This implementation is heavily based on simple Diffie-Hellman guides and a simplified version of the Signal protocl (https://signal.org/docs/)

use rand::{rngs::OsRng, RngCore};
use x25519_dalek::{StaticSecret, PublicKey};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

pub struct KeyPair {
    pub secret: StaticSecret,
    pub public: PublicKey,
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

// Generates the user's public and private keys
pub fn gen_keys() -> KeyPair {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    KeyPair { secret, public }
}

// Converts the private key to bytes for easier network operations
pub fn public_bytes(public: &PublicKey) -> [u8; 32] {
    public.to_bytes()
}

// Converts bytes back into a public key
pub fn public_from_bytes(bytes: [u8; 32]) -> PublicKey {
    PublicKey::from(bytes)
}

pub fn group_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

// Diffie-Hellman exchange
pub fn get_shared_key(private: &StaticSecret, ext_public: &PublicKey) -> [u8; 32] {
    let shared = private.diffie_hellman(ext_public);
    let key_deriver = Hkdf::<Sha256>::new(None, shared.as_bytes());
    let mut out = [0u8; 32];
    
    // HKDF key derivation to turn diffie-hellman output into a usable AES key
    key_deriver.expand(&[], &mut out).expect("Key derivation failed");
    drop(shared); // <-- Not sure if this is necessary or if x25519-dalek already does it
    out
}