use axum::{
    routing::post,
    Router,
};

use crate::{app_state::AppState, http::handlers::file::create::{create_file_handler}};

pub fn file_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_file_handler))
}
