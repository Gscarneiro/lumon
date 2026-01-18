use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use crate::app_state::AppState;

pub async fn signup_handler(State(_state): State<AppState>) -> impl IntoResponse {
    //do signup shit
    (StatusCode::OK, "ok")
}
