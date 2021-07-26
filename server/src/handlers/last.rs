use crate::redis::Redis;
use tracing::debug;
use warp::Rejection;

#[tracing::instrument(skip(redis))]
pub async fn get_last_price(ticker: String, redis: Redis) -> Result<impl warp::Reply, Rejection> {
    let key = format!("price/{}", ticker);
    let price = redis.get::<Option<f64>>(&key).await?;
    debug!(?price);
    Ok(warp::reply::json(&price))
}

#[tracing::instrument(skip(redis))]
pub async fn get_last_open(ticker: String, redis: Redis) -> Result<impl warp::Reply, Rejection> {
    let key = format!("open/{}", ticker);
    let price = redis.get::<Option<f64>>(&key).await?;
    debug!(?price);
    Ok(warp::reply::json(&price))
}

#[tracing::instrument(skip(redis))]
pub async fn get_last_close(ticker: String, redis: Redis) -> Result<impl warp::Reply, Rejection> {
    let key = format!("close/{}", ticker);
    let price = redis.get::<Option<f64>>(&key).await?;
    debug!(?price);
    Ok(warp::reply::json(&price))
}
