use tokio::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

mod config;
mod app_state;
mod http;
mod services;
mod db;

use config::Config;
use app_state::AppState;
use http::router::create_router;

use services::auth_service::AuthService;
use db::repositories::users_repo::UserRepository;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await
    .expect("Failed to create pool.");

    let state = AppState::new(AuthService::new(UserRepository::new(pool.clone())));

    let app = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Failed to bind to address.");

    axum::serve(listener, app)
    .await
    .expect("Server Error");
}
