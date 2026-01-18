use axum::{
    routing::post,
    Router,
};

use crate::{app_state::AppState, http::handlers::auth::{signup::signup_handler, login::login_handler}};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
}
