use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub innie_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub seed: i64,
    pub target_per_bin: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinStatus {
    Open,
    Full,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bin {
    pub id: Uuid,
    pub file_id: Uuid,
    pub bin_index: i32,
    pub filled_count: i32,
    pub status: BinStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Classification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub file_id: Uuid,
    pub bin_id: Uuid,
    pub numbers: serde_json::Value,
    pub score: i32,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}