#![allow(unused)]

use sqlx::PgPool;

use crate::services::{auth_service::AuthService, token_service::TokenService};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub token_service: TokenService,
}

impl AppState {
    pub fn new(auth_service: AuthService, token_service: TokenService) -> Self {
        Self { auth_service, token_service }
    }
}
