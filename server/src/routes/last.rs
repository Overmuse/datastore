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
        .and_then(handlers::last::get_last_price)
}

pub fn get_last_open(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("open" / String)
        .and(warp::get())
        .and(with_redis(redis))
        .and_then(handlers::last::get_last_open)
}

pub fn get_last_close(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("close" / String)
        .and(warp::get())
        .and(with_redis(redis))
        .and_then(handlers::last::get_last_close)
}

pub fn last_routes(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_last_price(redis.clone())
        .or(get_last_open(redis.clone()))
        .or(get_last_close(redis))
}
