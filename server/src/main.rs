use anyhow::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

mod db;
mod error;
mod handlers;
mod routes;
use db::DbPool;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    set_global_default(subscriber)?;
    let db = DbPool::new()?;
    db.migrate().await?;
    let routes = routes::routes(db);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
