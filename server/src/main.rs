use anyhow::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Subscriber;

mod db;
mod error;
mod handlers;
mod routes;
use db::DbPool;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = Subscriber::new();
    set_global_default(subscriber)?;
    let db = DbPool::new()?;
    db.migrate().await?;
    let routes = routes::routes(db);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
