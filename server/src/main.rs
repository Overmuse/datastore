mod db;
mod error;
mod handlers;
mod routes;
use db::DbPool;

#[tokio::main]
async fn main() -> Result<(), mobc::Error<tokio_postgres::Error>> {
    let db = DbPool::new()?;
    let routes = routes::routes(db);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
