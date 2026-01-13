use sqlx::{PgPool, Error};
use sqlx::{query_as, query};
use crate::db::models::Session;
use uuid::Uuid;

pub struct SessionRepository {
    pool: PgPool
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<Session, Error> {
        let session = query_as::<_, Session>("
            INSERT INTO sessions (user_id, started_at, ended_at)
            VALUES ($1, NOW(), NULL)
            RETURNING id, user_id, started_at, ended_at
        ")
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn get_active_by_user(&self, user_id: Uuid) -> Result<Option<Session>, Error> {
        let session = query_as::<_, Session>("
            SELECT id, user_id, started_at, ended_at
            FROM sessions
            WHERE user_id = $1 AND ended_at IS NULL
        ")
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn get_session_by_id(&self, session_id: Uuid) -> Result<Option<Session>, Error> {
        let session = query_as::<_, Session>("
            SELECT id, user_id, started_at, ended_at
            FROM sessions
            WHERE id = $1
        ")
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn end_session(&self, session_id: Uuid) -> Result<bool, Error> {
        let result = query("
            UPDATE sessions
            SET ended_at = NOW()
            WHERE id = $1 AND ended_at IS NULL
        ")
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}