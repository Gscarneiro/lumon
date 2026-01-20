use tokio::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
use axum::{Router};

mod config;
mod app_state;
mod http;
mod services;
mod db;

use config::Config;
use app_state::AppState;
use http::router::create_router;

use services::{
    auth_service::AuthService, 
    hash_service::HashService, 
    token_service::TokenService,
    file_service::FileService,
};

use db::repositories::{
    users_repo::UserRepository,
    files_repo::FileRepository,
    bins_repo::BinRepository
};

#[tokio::main]
async fn main() {
    let config = Config::load();

    let pool = init_pool(&config.database_url).await;

    let state = init_state(pool, config.jwt_secret);

    let app = create_router(state);

    serve(app).await;
}

async fn init_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool.")
}

fn init_state(pool: PgPool, jwt_secret: String) -> AppState {

    let user_repo = UserRepository::new(pool.clone());
    let hash_service = HashService::new();
    let auth_service = AuthService::new(user_repo, hash_service);
    
    let token_service = TokenService::new(jwt_secret);

    let file_repo = FileRepository::new(pool.clone());
    let bins_repo = BinRepository::new(pool.clone());
    let file_service = FileService::new(file_repo, bins_repo);

    AppState::new(auth_service, token_service, file_service)
}

async fn serve(app: Router) {

    let listener = TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Failed to bind to address.");

    axum::serve(listener, app)
    .await
    .expect("Server Error");
}
