use anyhow::Result;
use kafka_settings::consumer;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

mod db;
mod error;
mod handlers;
mod redis;
mod relay;
mod routes;
mod settings;
use crate::redis::Redis;
use db::DbPool;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let subscriber = Subscriber::builder()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    set_global_default(subscriber)?;
    let settings = Settings::new()?;
    let db = DbPool::new(settings.database.url, settings.database.name)?;
    db.migrate().await?;
    let redis = Redis::new(settings.redis)?;
    let redis2 = redis.clone();
    let routes = routes::routes(db, redis);
    let consumer = consumer(&settings.kafka)?;
    let relay = relay::Relay::new(consumer, redis2);
    tokio::spawn(async move { relay.run().await });

    warp::serve(routes)
        .run(([0, 0, 0, 0], settings.webserver.port))
        .await;
    Ok(())
}
