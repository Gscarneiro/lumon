use sqlx::{PgPool, Error};
use sqlx::{query_as, query};
use crate::db::models::User;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, email: &str, innie_name: &str, password_hash: &str) -> Result<User, Error> {
        let user = query_as::<_, User>("
            INSERT INTO users (email, innie_name, password_hash, is_active, created_at)
            VALUES ($1, $2, $3, TRUE, NOW())
            RETURNING id, email, innie_name, password_hash, is_active, created_at
        ")
        .bind(email)
        .bind(innie_name)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>{
        let user = query_as::<_, User>("
            SELECT id, email, innie_name, password_hash, is_active, created_at
            FROM users
            WHERE email = $1 and is_active = TRUE;
        ")
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, Error> {
        let user = query_as::<_, User>("
            SELECT id, email, innie_name, password_hash, is_active, created_at
            FROM users
            WHERE id = $1
        ")
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn deactivate_user(&self, user_id: uuid::Uuid) -> Result<bool, Error> {
        let result = query("
            UPDATE users
            SET is_active = FALSE
            WHERE id = $1
        ")
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn email_exists(&self, email: &str) -> Result<bool, Error> {
        let count: (i64,) = query_as::<_, (i64,)>("
            SELECT COUNT(*) FROM users WHERE email = $1
        ")
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 > 0)
    }
}