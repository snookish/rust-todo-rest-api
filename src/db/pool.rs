use std::env::{self, VarError};

use diesel::{
    MysqlConnection,
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to load environment configuration: {0}")]
    Dotenv(#[from] dotenvy::Error),

    #[error("DATABASE_URL environment variable is missing or invalid: {0}")]
    Environment(#[from] VarError),

    #[error("Database connection pool error: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),

    #[error("DATABASE_URL can't be empty")]
    EmptyDatabaseURL,
}

pub fn create_db_pool() -> Result<DbPool, DbError> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(DbError::Dotenv(error)),
    }

    let database_url = env::var("DATABASE_URL")?;
    if database_url.trim().is_empty() {
        return Err(DbError::EmptyDatabaseURL);
    }

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(100)
        .min_idle(Some(5))
        .connection_timeout(std::time::Duration::from_secs(10))
        .test_on_check_out(true)
        .build(manager)?;

    Ok(pool)
}
