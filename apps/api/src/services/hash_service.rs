use argon2::{
    Argon2, password_hash::{ 
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier,
        SaltString,
        Error as PasswordHashError,
        rand_core::OsRng 
    }
};

#[derive(Clone)]
pub struct HashService {
    argon2: Argon2<'static>,
}

impl HashService {
    pub fn new() -> Self {
        Self { 
            argon2: Argon2::default() 
        }
    }

    pub fn hash(&self, password: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify(&self, password: &str, password_hash: &str) -> Result<bool, PasswordHashError> {
        let parsed_hash = PasswordHash::new(password_hash)?;

        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}