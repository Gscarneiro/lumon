use uuid::Uuid;
use serde_json::Value;
use sqlx::{PgPool, Error, query_as};

use crate::db::models::File;
#[derive(Clone)]
pub struct FileRepository {
    pool: PgPool
}

impl FileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_file(&self, name: &str, seed: i64, min_fill: f64, tolerance: f64, dominance_gap: Option<f64>, target_profile: Value) -> Result<File, Error> {
        let file = query_as::<_, File>("
            INSERT INTO files (name, seed, target_profile, min_fill, tolerance, dominance_gap)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, seed, target_profile, min_fill, tolerance, dominance_gap, created_at
        ")
        .bind(name)
        .bind(seed)
        .bind(target_profile)
        .bind(min_fill)
        .bind(tolerance)
        .bind(dominance_gap)
        .fetch_one(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, Error> {
        let file = query_as::<_, File>("
            SELECT id, name, seed, target_per_bin, created_at
            FROM files
            WHERE id = $1
        ")
        .bind(file_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<File>, Error> {
        let file = query_as::<_, File>("
            SELECT id, name, seed, target_per_bin, created_at
            FROM files
            WHERE name = $1
        ")
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn list_files(&self) -> Result<Vec<File>, Error> {
        let files = query_as::<_, File>("
            SELECT id, name, seed, target_per_bin, created_at
            FROM files
            ORDER BY created_at DESC
        ")
        .fetch_all(&self.pool)
        .await?;

        Ok(files)
    }
}