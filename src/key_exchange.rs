use rand::rngs::OsRng;
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

pub fn gen_keys() -> KeyPair {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    KeyPair { secret, public }
}

pub fn public_bytes(public: &PublicKey) -> [u8; 32] {
    public.to_bytes()
}

pub fn public_from_bytes(bytes: [u8; 32]) -> PublicKey {
    PublicKey::from(bytes)
}

pub fn get_shared_key(private: &StaticSecret, ext_public: &PublicKey) -> [u8; 32] {
    let shared = private.diffie_hellman(ext_public);
    let key_deriver = Hkdf::<Sha256>::new(None, shared.as_bytes());
    let mut out = [0u8; 32];
    key_deriver.expand(&[], &mut out).expect("Key derivation failed");
    drop(shared);
    out
}