use rand::rngs::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};
use hkdf::Hkdf;
use sha2::Sha256;