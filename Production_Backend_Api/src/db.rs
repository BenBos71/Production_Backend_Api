use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),
}

pub type DbPool = SqlitePool;

/// Initialize the database connection pool
pub async fn init_db_pool() -> Result<DbPool, DbError> {
    let database_url = env::var("DATABASE_URL")?;
    
    // Create the database file if it doesn't exist
    if !std::path::Path::new("database.db").exists() {
        std::fs::File::create("database.db").map_err(|e| {
            DbError::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create database file: {}", e),
            )))
        })?;
    }
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| DbError::Database(e.into()))?;

    Ok(pool)
}

/// A state that holds the database connection pool
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

impl AppState {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}