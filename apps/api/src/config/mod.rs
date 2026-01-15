use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
 
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {

        dotenv().ok();

        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}