use std::result;

use crate::db::repositories::users_repo::UserRepository;
use crate::services::hash_service::HashService;
use super::auth_errors::SignupError;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    hash_service: HashService
}

impl AuthService {
    pub fn new(user_repo: UserRepository, hash_service: HashService) -> Self {
        Self { 
            user_repo, 
            hash_service 
        }
    }

    pub async fn signup(&self, email: &str, innie_name: &str, password: &str) -> result::Result<(), SignupError> {

        let exists = self.user_repo.email_exists(&email).await.map_err(|_| SignupError::DatabaseError)?;

        if exists {
            return Err(SignupError::EmailAlreadyExists);
        }

        let password_hash = self.hash_service.hash(password).map_err(|_| SignupError::HashingFailed)?;

        self.user_repo.create_user(email, &innie_name, &password_hash).await.map_err(|_| SignupError::DatabaseError)?;

        Ok(())
    }
}