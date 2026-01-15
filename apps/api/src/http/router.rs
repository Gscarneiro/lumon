use axum::{
    routing::get,
    Router,
};

use crate::app_state::AppState;
use crate::http::handlers::health::health_handler;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .with_state(state)
}
