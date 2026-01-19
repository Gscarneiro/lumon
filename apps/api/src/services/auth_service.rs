use std::result;

use crate::http::models::signup_request::SignupRequest;
use crate::db::repositories::users_repo::UserRepository;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository
}

impl AuthService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn signup(&self, body: SignupRequest) -> result::Result<(), SignupError> {
        if self.user_repo.email_exists(&body.email).await.unwrap_or(false) {
            // Handle error: email already exists
            return Err(SignupError::EmailAlreadyExists);
        }

        let password_hash = format!("hashed_{}", body.password); // Placeholder for actual hashing logic

        self.user_repo.create_user(&body.email, &body.innie_name, &password_hash).await;

        Ok(())
    }
}

//Achar lugar melhor pra colocar isso
#[derive(Debug)]
pub enum SignupError {
    EmailAlreadyExists,
    Database,
}