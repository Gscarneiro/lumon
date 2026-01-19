#![allow(unused)]

use sqlx::PgPool;

use crate::services::auth_service::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,

}

impl AppState {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }
}
