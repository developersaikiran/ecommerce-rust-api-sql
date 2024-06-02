use std::{env, sync::{Arc, Mutex}};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[allow(non_snake_case)]
pub struct AppState {
    pub db: Arc<Mutex<PgPool>>,
}

impl AppState {
    pub async fn init() -> Result<AppState, sqlx::Error> {
        dotenv().ok();
        
        let database_url = env::var(format!("DATABASE_URL_{}", env::var("RUN_MODE").unwrap_or_else(|_| "DEV".to_string()))).unwrap();
        // let database_url = std::env::var("DATABASE_URL").unwrap();
        println!("database_url! {}", database_url);

        let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await{
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };


        Ok(AppState {
            db: Arc::new(Mutex::new(pool)),
        })
    }
}
