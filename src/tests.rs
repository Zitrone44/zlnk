#![cfg(test)]
use r2d2;
use r2d2_redis::RedisConnectionManager;
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
    let long_url = "https://zlnk.de".to_string();
    let shorted = short(long_url.clone(), env, connection).unwrap();
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
    let long_url = "data:text/plain,https://zlnk.de".to_string();
    let shorted = short(long_url, env, connection).is_none();
    assert_eq!(shorted, true);
}
