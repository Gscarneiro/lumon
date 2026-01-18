use axum::{
    routing::get,
    Router,
};

use crate::app_state::AppState;
use crate::http::handlers::health::health_handler;

pub fn user_router() -> Router {
    Router::new()
        .route("", get(health_handler))
}
