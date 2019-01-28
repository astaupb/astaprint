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

use std::env;

#[derive(Debug, Clone)]
pub enum Redis
{
    Lock,
    Store,
    Dispatcher,
    Worker
}

pub fn create_redis_pool(url: &str, max_size: u32) -> Pool<RedisConnectionManager>
{
    Pool::builder()
        .max_size(max_size)
        .build(RedisConnectionManager::new(url).expect("creating Connection Manager"))
        .expect("creating Redis Connection Pool")
}

pub fn get_redis_pool(max_size: u32, redis: Redis) -> Pool<RedisConnectionManager>
{
    create_redis_pool(
        &match redis {
            Redis::Lock => {
                env::var("ASTAPRINT_LOCK_REDIS_URL")
                    .expect("reading ASTAPRINT_LOCK_REDIS_URL from env")
            },
            Redis::Store => {
                env::var("ASTAPRINT_STORE_REDIS_URL")
                    .expect("reading ASTAPRINT_STORELOCK_REDIS_URL from env")
            },
            Redis::Dispatcher => {
                env::var("ASTAPRINT_DISPATCHER_REDIS_URL")
                    .expect("reading ASTAPRINT_DISPATCHER_REDIS_URL from env")
            },
            Redis::Worker => {
                env::var("ASTAPRINT_WORKER_REDIS_URL")
                    .expect("reading ASTAPRINT_WORKER_REDIS_URL from env")
            },
        },
        max_size,
    )
}
