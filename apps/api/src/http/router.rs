use axum::{
    routing::get,
    Router,
};

use crate::app_state::AppState;
use crate::http::handlers::health::health_handler;

use crate::http::routers::{auth::auth_router, file::file_router};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/auth", auth_router())
        .nest("/file", file_router())
        .with_state(state)
}
