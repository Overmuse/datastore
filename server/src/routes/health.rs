use crate::handlers;
use crate::redis::with_redis;
use crate::Redis;
use warp::Filter;

pub fn health_check(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health_check")
        .and(warp::get())
        .and(with_redis(redis))
        .and_then(handlers::health::health_check)
}

pub fn health_routes(
    redis: Redis,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check(redis.clone())
}
