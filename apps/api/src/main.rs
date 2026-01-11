#![allow(unused)] 

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

mod config;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool.");

    let row: PgRow = sqlx::query("SELECT 'Connected to Postgres' AS message")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch row.");

        println!("{}", row.get::<&str, _>(0));
}
