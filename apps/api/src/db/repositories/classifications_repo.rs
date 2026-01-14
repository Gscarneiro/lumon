use sqlx::{PgPool, Error, query_as};
use crate::db::models::Classification;
use uuid::Uuid;

pub struct ClassificationRepository {
    pool: PgPool
}

impl ClassificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_classification(&self, classification: &Classification) -> Result<Classification, Error> {
        let classification = query_as::<_, Classification>("
            INSERT INTO classifications (user_id, session_id, file_id, bin_id, numbers, score, tags)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, session_id, file_id, bin_id, numbers, score, tags, created_at
        ")
        .bind(classification.user_id)
        .bind(classification.session_id)
        .bind(classification.file_id)
        .bind(classification.bin_id)
        .bind(&classification.numbers)
        .bind(classification.score)
        .bind(&classification.tags)
        .fetch_one(&self.pool)
        .await?;

        Ok(classification)
    }

    pub async fn get_by_id(&self, classification_id: Uuid) -> Result<Option<Classification>, Error> {
        let classification = query_as::<_, Classification>("
            SELECT id, user_id, session_id, file_id, bin_id, numbers, score, tags, created_at
            FROM classifications
            WHERE id = $1
        ")
        .bind(classification_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(classification)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Classification>, Error> {
        let classifications = query_as::<_, Classification>("
            SELECT id, user_id, session_id, file_id, bin_id, numbers, score, tags, created_at
            FROM classifications
            WHERE user_id = $1
            ORDER BY created_at DESC
        ")
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(classifications)
    }

    pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<Classification>, Error> {
        let classifications = query_as::<_, Classification>("
            SELECT id, user_id, session_id, file_id, bin_id, numbers, score, tags, created_at
            FROM classifications
            WHERE session_id = $1
            ORDER BY created_at DESC
        ")
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(classifications)
    }

    pub async fn list_by_file(&self, file_id: Uuid) -> Result<Vec<Classification>, Error> {
        let classifications = query_as::<_, Classification>("
            SELECT id, user_id, session_id, file_id, bin_id, numbers, score, tags, created_at
            FROM classifications
            WHERE file_id = $1
            ORDER BY created_at DESC
        ")
        .bind(file_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(classifications)
    }
}