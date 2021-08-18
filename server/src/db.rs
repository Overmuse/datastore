use crate::error::Error;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};
use warp::Filter;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

type DbCon = Connection<PgConnectionManager<NoTls>>;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[derive(Clone)]
pub struct DbPool {
    inner: Pool<PgConnectionManager<NoTls>>,
}

impl DbPool {
    pub fn new(
        address: String,
        db_name: String,
    ) -> Result<DbPool, mobc::Error<tokio_postgres::Error>> {
        let full_address = format!("{}/{}", address, db_name);
        let config = Config::from_str(&full_address)?;

        let manager = PgConnectionManager::new(config, NoTls);
        let inner = Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager);
        Ok(Self { inner })
    }

    pub async fn get_connection(&self) -> Result<DbCon, Error> {
        self.inner.get().await.map_err(Error::DbPool)
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        let mut connection = self.get_connection().await?;
        embedded::migrations::runner()
            .run_async(connection.deref_mut())
            .await
            .map_err(Error::DbMigrate)?;
        Ok(())
    }
}

pub fn with_db(db: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
