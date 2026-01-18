use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use crate::app_state::AppState;

pub async fn login_handler(State(_state): State<AppState>) -> impl IntoResponse {
    //do login shit
    (StatusCode::OK, "ok")
}
