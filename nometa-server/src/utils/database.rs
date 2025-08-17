use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


#[derive(Deserialize, Debug)]
pub(crate) struct PostgreSQLConfig {
    app_user: String,
    app_password: String,
    host: String,
    port: u16,
    db: String,
}

impl PostgreSQLConfig {
    pub(crate) fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("POSTGRES_").from_env()
    }

    fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.app_user, self.app_password, self.host, self.port, self.db
        )
    }
}

pub(crate) struct PostgreSQL {
    pool: Pool<Postgres>,
}

impl PostgreSQL {
    /// Create a new PostgreSQL pool connection
    pub(crate) async fn new(config: PostgreSQLConfig) -> Result<Self, sqlx::Error> {
        let min_connections = if cfg!(debug_assertions) { 5 } else { 10 };
        let max_connections = if cfg!(debug_assertions) { 15 } else { 20 };
        
        let pool = PgPoolOptions::new()
            .min_connections(min_connections)
            .max_connections(max_connections)
            .connect(&config.url())
            .await?;

        Ok(Self { pool })
    }

    /// Get pool
    pub(crate) fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }
}
