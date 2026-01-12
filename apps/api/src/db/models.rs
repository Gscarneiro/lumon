use serde::{Deserialize, Serialize};
use chrono::{Datetime, Utc};
use uuid:Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub innie_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: Datetime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub started_at: Datetime<Utc>,
    pub ended_at: Datetime<Utc>,
}