use crate::error::Error;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};
use warp::Filter;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

type DbCon = Connection<PgConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct DbPool {
    inner: Pool<PgConnectionManager<NoTls>>,
}

impl DbPool {
    pub fn new() -> Result<DbPool, mobc::Error<tokio_postgres::Error>> {
        let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

        let manager = PgConnectionManager::new(config, NoTls);
        let inner = Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager);
        Ok(Self { inner })
    }

    pub async fn get_connection(&self) -> Result<DbCon, Error> {
        self.inner.get().await.map_err(Error::DbPoolError)
    }
}

pub fn with_db(db: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
