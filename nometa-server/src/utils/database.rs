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

    // /// Create a PostgreSQL URL from environment variables and docker secrets
    // fn build_url_from_env() -> String {
    //     let env_vars = match cfg!(debug_assertions) {
    //         true => vec!["POSTGRES_USER_DEV", "POSTGRES_PASSWORD_DEV", "POSTGRES_HOST_DEV", "POSTGRES_PORT_DEV", "POSTGRES_DB_DEV"],
    //         false => vec!["POSTGRES_USER", "POSTGRES_PASSWORD", "POSTGRES_HOST", "POSTGRES_PORT", "POSTGRES_DB"]
    //     };

    //     let mut env_values = Vec::new();
    //     for var in env_vars {
    //         let value = std::env::var(var)
    //             .expect(&format!("{} must be set", var));

    //         env_values.push(value);
    //     }

    //     format!(
    //         "postgresql://{}:{}@{}:{}/{}",
    //         env_values[0], env_values[1], env_values[2], env_values[3], env_values[4]
    //     )
    // }
}
