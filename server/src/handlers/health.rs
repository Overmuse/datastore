use super::last::get_last_price;
use crate::redis::Redis;
use warp::Rejection;

#[tracing::instrument(skip(redis))]
pub async fn health_check(redis: Redis) -> Result<impl warp::Reply, Rejection> {
    get_last_price("AAPL".to_string(), redis).await?;
    Ok(warp::reply())
}
