use axum::{
    extract::{State, Json},
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    services::auth_errors::AuthErrors,
};


use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub innie_name: String,
    pub password: String,
}

pub async fn signup_handler(State(state): State<AppState>, Json(payload): Json<SignupRequest>) -> StatusCode {
    match state.auth_service.signup(&payload.email, &payload.innie_name, &payload.password).await {
        Ok(_) => StatusCode::CREATED,
        Err(AuthErrors::EmailAlreadyExists) => StatusCode::CONFLICT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
