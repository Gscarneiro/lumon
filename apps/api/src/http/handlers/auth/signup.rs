use axum::{
    extract::{State, Json},
    http::StatusCode,
};

use crate::app_state::AppState;

pub async fn signup_handler(State(state): State<AppState>, Json(payload): Json<crate::http::models::signup_request::SignupRequest>) -> StatusCode {
    match state.auth_service.signup(payload).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
