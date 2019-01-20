#[macro_use]
extern crate log;
extern crate bincode;
extern crate r2d2_redis;
extern crate serde;
extern crate threadpool;

pub mod lock;
pub mod queue;
pub mod store;

use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

pub fn create_redis_pool(url: &str, max_size: u32) -> Pool<RedisConnectionManager>
{
    Pool::builder()
        .max_size(max_size)
        .build(RedisConnectionManager::new(url).expect("creating Connection Manager"))
        .expect("creating Redis Connection Pool")
}

