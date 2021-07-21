use crate::handlers;
use crate::redis::with_redis;
use crate::Redis;
use warp::Filter;

pub fn get_last_price(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("last" / String)
        .and(warp::get())
        .and(with_redis(redis))
        .and_then(|ticker, redis| handlers::last::get_last_price(ticker, redis))
}

pub fn last_routes(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_last_price(redis)
}
