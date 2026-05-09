use std::env;
use sqlx::{Error, PgPool};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
        let pool = PgPool::connect(&database_url).await?;

        Ok(
            Self {
                pool
            }
        )
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
