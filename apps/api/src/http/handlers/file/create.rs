use axum::{
    extract::{Json, State},
    response::IntoResponse,
};

use crate::{
    app_state::AppState,
};

use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateFileRequest {
    pub name: String,
    pub seed: i64,
    pub min_fill: f64,
    pub tolerance: f64,
    pub target_profile: TargetProfile,
}

#[derive(Deserialize)]
pub struct TargetProfile {
    pub woe: f64,
    pub frolic: f64,
    pub dread: f64,
    pub malice: f64,
}

pub async fn create_file_handler(State(state): State<AppState>, Json(payload): Json<CreateFileRequest>) -> impl IntoResponse {
    state.file_service.create_file(&payload.name, payload.seed, payload.min_fill, payload.tolerance, None,
        serde_json::json!({
            "woe": payload.target_profile.woe,
            "frolic": payload.target_profile.frolic,
            "dread": payload.target_profile.dread,
            "malice": payload.target_profile.malice,
        })
    ).await;
}
