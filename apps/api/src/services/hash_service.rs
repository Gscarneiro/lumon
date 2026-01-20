use argon2::{
    password_hash::{ rand_core::OsRng, PasswordHasher, SaltString, PasswordHash, PasswordVerifier },
    Argon2
};

#[derive(Clone)]
pub struct HashService {
}

impl HashService {
    pub fn new() -> Self {
        Self { }
    }

    pub async fn hash(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
    }

    pub async fn verify(&self, password: &str, password_hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();

        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}