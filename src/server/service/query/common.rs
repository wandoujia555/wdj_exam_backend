use crate::service::redis::GLOBAL_REDIS;
use redis::{Commands, RedisError, ToRedisArgs};
pub const REDIS_PAPER_KEY: &str = "paper_";
pub const REDIS_CLASS_KEY: &str = "class_";
// const REDIS_KEY: &str = "class_";
use std::fmt;

// get redis key by key
pub async fn get_string_by_key_redis<T: ToString,U: ToString>(redus_key: T, key: U) -> Result<String, RedisError> {
    let mut con = GLOBAL_REDIS
        .lock()
        .await
        .get()
        .expect("Failed to get Redis connection");

    let key = format!("{}{}", redus_key.to_string(), key.to_string());
    let result: Result<String, _> = con.get(key);
    drop(con);
    result
}



pub async fn set_string_by_key_redis<T: ToString,U: ToString, V: ToRedisArgs>(redus_key: T, key: U, value: V) {
    let mut con = GLOBAL_REDIS
        .lock()
        .await
        .get()
        .expect("Failed to get Redis connection");
    let key = format!("{}{}", redus_key.to_string(), key.to_string());
    let _: () = con
        .set(key, value)
        .unwrap();
    drop(con);
}