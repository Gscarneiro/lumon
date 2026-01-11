use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
 
    pub database_url: String,
    pub environment: String,
}

impl Config {
    pub fn load() -> Self {

        dotenv().ok();

        Self {
            database_url: std::env::var("DB_URL").expect("DB_URL must be set"),
            environment: std::env::var("ENVIRONMENT").expect("ENVIRONMENT must be set"),
        }
    }
}

fn read_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!("Environment variable '{}' is not set", key);
    })
}