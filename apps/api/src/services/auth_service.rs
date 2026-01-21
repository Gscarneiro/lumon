use std::result;
use uuid::Uuid;

use crate::db::repositories::users_repo::UserRepository;
use crate::services::hash_service::HashService;
use super::errors::AuthErrors;

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

    pub async fn signup(&self, email: &str, innie_name: &str, password: &str) -> result::Result<(), AuthErrors> {

        let exists = self.user_repo.email_exists(&email).await.map_err(|_| AuthErrors::DatabaseError)?;

        if exists {
            return Err(AuthErrors::EmailAlreadyExists);
        }

        let password_hash = self.hash_service.hash(password).map_err(|_| AuthErrors::HashingFailed)?;

        self.user_repo.create_user(email, &innie_name, &password_hash).await.map_err(|_| AuthErrors::DatabaseError)?;

        Ok(())
    }

    pub async fn login(&self, email: &str, password: &str) -> result::Result<Uuid, AuthErrors> {

        let user = self.user_repo.get_user_by_email(&email).await.map_err(|_| AuthErrors::DatabaseError)?;

        match user {
            Some(u) => {
                let valid = self.hash_service.verify(password, &u.password_hash).map_err(|_| AuthErrors::InvalidCredentials)?;

                if !valid {
                    return Err(AuthErrors::InvalidCredentials);
                }

                Ok(u.id)
            },
            None => return Err(AuthErrors::InvalidCredentials),
        }
    }
}