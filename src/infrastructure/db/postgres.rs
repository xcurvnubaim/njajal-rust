use axum::async_trait;
use sqlx::PgPool;
use std::error::Error;

pub struct Database {

}

#[async_trait]
pub trait DatabaseTrait {
    async fn init_pool(database_url: &str) -> Result<PgPool, Box<dyn Error>>;
}

#[async_trait]
impl DatabaseTrait for Database {
/// Initializes and returns a PostgreSQL connection pool.
///
/// # Arguments
///
/// * `database_url` - A string slice that holds the database URL.
///
/// # Returns
///
/// Returns a `Result` containing the `PgPool` on success or an error on failure.
    async fn init_pool(database_url: &str) -> Result<PgPool, Box<dyn Error>> {
        // Attempt to create a connection pool
        let pool = PgPool::connect(database_url).await?;
        
        // Return the connection pool
        Ok(pool)
    }
}