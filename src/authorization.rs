use argon2::Argon2;
// https://docs.rs/argon2/latest/argon2/
use password_hash::{PasswordHasher, PasswordHash, PasswordVerifier, SaltString};

pub struct AuthenticationManager {
    regular_password_hash: String,
    panic_password_hash: String,
}

impl AuthenticationManager {
    
    pub fn new(regular_pass: &str, panic_pass: &str) -> Self {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(rand::thread_rng());
        
        let regular_hash = argon2
            .hash_password(regular_pass.as_bytes(), &salt).unwrap().to_string();
        
        let salt2 = SaltString::generate(rand::thread_rng());
        let panic_hash = argon2
            .hash_password(panic_pass.as_bytes(), &salt2).unwrap().to_string();
        
        Self {
            regular_password_hash: regular_hash,
            panic_password_hash: panic_hash,
        }
    }


    pub fn verify_password(&self, input: &str) -> bool{
        let argon2 = Argon2::default();
        
        // check regular password
        if let Ok(parsed) = PasswordHash::new(&self.regular_password_hash) {
            if argon2.verify_password(input.as_bytes(), &parsed).is_ok() {
                return true;
            }
        }
        
        // check panic password
        if let Ok(parsed) = PasswordHash::new(&self.panic_password_hash) {
            if argon2.verify_password(input.as_bytes(), &parsed).is_ok() {
                return true;
            }
        }
        false
    }
}   