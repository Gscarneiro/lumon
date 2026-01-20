use jsonwebtoken::{EncodingKey, Header};
use serde::{Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct TokenService {
    secret: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

impl TokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate(&self, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + 900,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }
}
