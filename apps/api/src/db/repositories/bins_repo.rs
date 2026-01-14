use sqlx::{PgPool, Error, query_as, query};
use crate::db::models::Bin;
use uuid::Uuid;

pub struct BinRepository {
    pool: PgPool
}

impl BinRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_bins(&self, file_id: Uuid, quantity: Option<i32>) -> Result<(), Error> {
        
        let quantity = quantity.unwrap_or(5);

        for bin_index in 0..quantity {

            let bin = query_as::<_, Bin>("
                INSERT INTO bins (file_id, bin_index, filled_count, status)
                VALUES ($1, $2, 0, 'open')
                RETURNING id, file_id, bin_index, filled_count, status, created_at
            ")
            .bind(file_id)
            .bind(bin_index)
            .fetch_one(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_by_id(&self, bin_id: Uuid) -> Result<Option<Bin>, Error> {
        let bin = query_as::<_, Bin>("
            SELECT id, file_id, bin_index, filled_count, status, created_at
            FROM bins
            WHERE id = $1
        ")
        .bind(bin_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(bin)
    }

    pub async fn list_bins(&self, file_id: Uuid) -> Result<Vec<Bin>, Error> {
        let file = query_as::<_, Bin>("
            SELECT id, file_id, bin_index, filled_count, status, created_at
            FROM bins
            WHERE file_id = $1
            ORDER BY bin_index ASC
        ")
        .bind(file_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn add_count(&self, bin_id: Uuid) -> Result<bool, Error> {
        let result = query("
            UPDATE bins
            SET filled_count = filled_count + 1
            WHERE id = $1 and status = 'open'
        ")
        .bind(bin_id)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
