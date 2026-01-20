use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app_state::AppState,
    services::auth_errors::AuthErrors
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
}

pub async fn login_handler(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    match state
        .auth_service
        .login(&payload.email, &payload.password)
        .await
    {
        Ok(user_id) => {
            match state.token_service.generate(&user_id.to_string()) {
                Ok(token) => {
                    let body = LoginResponse {
                        access_token: token,
                        token_type: "Bearer",
                        expires_in: 900,
                    };

                    (StatusCode::OK, Json(body)).into_response()
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }

        Err(AuthErrors::InvalidCredentials) => {
            StatusCode::UNAUTHORIZED.into_response()
        }

        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
