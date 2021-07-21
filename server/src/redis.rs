use crate::error::Error;
use crate::settings::RedisSettings;
use mobc::{Connection, Pool};
use mobc_redis::RedisConnectionManager;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use std::convert::Infallible;
use std::time::Duration;
use warp::Filter;

#[derive(Clone)]
pub struct Redis {
    pool: Pool<RedisConnectionManager>,
}

impl Redis {
    pub fn new(settings: RedisSettings) -> Result<Self, Error> {
        let client = redis::Client::open(settings.url)?;
        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder()
            .get_timeout(Some(Duration::from_secs(1)))
            .max_open(16)
            .max_idle(8)
            .max_lifetime(Some(Duration::from_secs(60)))
            .build(manager);
        Ok(Self { pool })
    }

    async fn get_connection(&self) -> Result<Connection<RedisConnectionManager>, Error> {
        self.pool.get().await.map_err(Error::RedisPoolError)
    }

    pub async fn get<T: Send + FromRedisValue>(&self, key: &str) -> Result<T, Error> {
        let mut con = self.get_connection().await?;
        con.get::<&str, T>(key).await.map_err(Error::RedisError)
    }

    pub async fn set<T: ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> Result<(), Error> {
        let mut con = self.get_connection().await?;
        con.set(key, value).await.map_err(Error::RedisError)
    }
}

pub fn with_redis(redis: Redis) -> impl Filter<Extract = (Redis,), Error = Infallible> + Clone {
    warp::any().map(move || redis.clone())
}
