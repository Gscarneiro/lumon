use std::result;

use crate::db::repositories::users_repo::UserRepository;
use crate::services::hash_service::HashService;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    hash_service: HashService
}

impl AuthService {
    pub fn new(user_repo: UserRepository, hash_service: HashService) -> Self {
        Self { user_repo, hash_service }
    }

    pub async fn signup(&self, email: &str, innie_name: &str, password: &str) -> result::Result<(), SignupError> {
        if self.user_repo.email_exists(&email).await.unwrap_or(false) {
            // Handle error: email already exists
            return Err(SignupError::EmailAlreadyExists);
        }

        let password_hash = self.hash_service.hash(password).await;

        self.user_repo.create_user(email, &innie_name, &password_hash).await.unwrap();

        Ok(())
    }
}

//Achar lugar melhor pra colocar isso
#[derive(Debug)]
pub enum SignupError {
    EmailAlreadyExists
}