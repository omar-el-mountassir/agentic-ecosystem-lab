//! Storage - Database interactions for persistence

use sqlx::postgres::PgPool;

pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub async fn new(database_url: &str) -> crate::Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
    
    pub async fn initialize(&self) -> crate::Result<()> {
        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}
