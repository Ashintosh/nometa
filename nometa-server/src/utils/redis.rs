use std::sync::Arc;

use dotenvy::dotenv;
use redis::AsyncCommands;
use redis::Client;
use redis::RedisError;
use redis::RedisResult;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub(crate) struct RedisConfig {
    host: String,
    port: u16,
}

impl RedisConfig {
    pub(crate) fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("REDIS_").from_env()
    }

    fn url(&self) -> String {
        format!(
            "redis://{}:{}",
            self.host, self.port
        )
    }
}

#[derive(Clone)]
pub(crate) struct RedisPool {
    client: Arc<Client>,
}

impl RedisPool {
    pub(crate) fn new(config: RedisConfig) -> Result<Self, RedisError> {
        let client = Client::open(config.url())?;
        Ok(Self { client: Arc::new(client) })
    }

    pub(crate) async fn set_session(&self, session_id: &str, user_id: i32, ttl_secs: u64) -> RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(session_id, user_id, ttl_secs).await
    }

    pub(crate) async fn get_session(&self, session_id: &str) -> RedisResult<Option<i32>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.get(session_id).await
    }
    
    pub(crate) async fn delete_session(&self, session_id: &str) -> RedisResult<()>{
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del(session_id).await
    }

    pub(crate) fn client(&self) -> Arc<Client> {
        self.client.clone()
    }
}