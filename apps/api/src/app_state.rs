#![allow(unused)]

use sqlx::PgPool;

use crate::services::{auth_service::AuthService, token_service::TokenService, file_service::FileService};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub token_service: TokenService,
    pub file_service: FileService,
}

impl AppState {
    pub fn new(auth_service: AuthService, token_service: TokenService, file_service: FileService) -> Self {
        Self { auth_service, token_service, file_service }
    }
}
