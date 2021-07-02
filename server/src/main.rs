use anyhow::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

mod db;
mod error;
mod handlers;
mod routes;
mod settings;
use db::DbPool;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    set_global_default(subscriber)?;
    let settings = Settings::new()?;
    let db = DbPool::new(settings.database.url, settings.database.name)?;
    db.migrate().await?;
    let routes = routes::routes(db);

    warp::serve(routes)
        .run(([0, 0, 0, 0], settings.webserver.port))
        .await;
    Ok(())
}
