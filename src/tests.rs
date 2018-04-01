#![cfg(test)]
use std::ops::Deref;
use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis;
use env_loader;
use shortener::{short, long};

#[test]
fn shortener_test() {
    let env = &env_loader::init();
    let manager = RedisConnectionManager::new(env.redis_url.as_str()).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    let connection = &pool.get().unwrap();
    let _:() = redis::cmd("FLUSHDB").query(connection.deref()).unwrap();
    let long_url = "https://zlnk.de".to_string();
    let shorted = short(long_url.clone(), env, connection, None).unwrap();
    let longed = long(shorted, connection).unwrap();
    assert_eq!(longed, long_url);
}

#[test]
fn invalid_url_test() {
    let env = &env_loader::init();
    let manager = RedisConnectionManager::new(env.redis_url.as_str()).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    let connection = &pool.get().unwrap();
    let _: () = redis::cmd("FLUSHDB").query(connection.deref()).unwrap();
    let long_url = "data:text/plain,https://zlnk.de".to_string();
    let shorted = short(long_url, env, connection, None).is_none();
    assert_eq!(shorted, true);
}

#[test]
fn double_shortening_test() {
    let env = &env_loader::init();
    let manager = RedisConnectionManager::new(env.redis_url.as_str()).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    let connection = &pool.get().unwrap();
    let _: () = redis::cmd("FLUSHDB").query(connection.deref()).unwrap();
    let long_url = "https://zlnk.de".to_string();
    let shorted_one = short(long_url.clone(), env, connection, None).unwrap();
    let shorted_two = short(long_url.clone(), env, connection, None).unwrap();
    assert_eq!(shorted_one, shorted_two);
}
