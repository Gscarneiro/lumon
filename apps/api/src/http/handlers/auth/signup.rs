use axum::{
    extract::{State, Json},
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    services::auth_errors::SignupError,
};

pub async fn signup_handler(State(state): State<AppState>, Json(payload): Json<crate::http::models::signup_request::SignupRequest>) -> StatusCode {
    match state.auth_service.signup(&payload.email, &payload.innie_name, &payload.password).await {
        Ok(_) => StatusCode::CREATED,
        Err(SignupError::EmailAlreadyExists) => StatusCode::CONFLICT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
