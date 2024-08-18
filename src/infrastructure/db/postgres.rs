use axum::async_trait;
use sqlx::PgPool;
use std::{env, error::Error};

pub struct Database {
    pool: PgPool,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init_pool() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn get_pool(&self) -> &PgPool;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init_pool() -> Result<Self, Box<dyn Error>> {
        // Fetch the DATABASE_URL environment variable
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}