use r2d2_redis::{
    r2d2::Pool,
    redis::{
        Commands,
        RedisResult,
    },
    RedisConnectionManager,
};

use serde::{
    de::DeserializeOwned,
    Serialize,
};

use threadpool::ThreadPool;

use std::{
    fmt::Debug,
    marker::PhantomData,
};

#[derive(Clone)]
pub struct TaskQueue<T, U>
{
    redis_pool: Pool<RedisConnectionManager>,
    thread_pool: ThreadPool,
    data: U,
    marker: PhantomData<T>,
    incoming: String,
    processing: String,
}

impl<T, U> TaskQueue<T, U>
where
    T: 'static + Serialize + DeserializeOwned + Debug,
    U: 'static + Send + Clone,
{
    pub fn new(
        name: &str,
        data: U,
        redis_pool: Pool<RedisConnectionManager>,
        thread_pool: ThreadPool,
    ) -> TaskQueue<T, U>
    {
        TaskQueue {
            marker: PhantomData::<T>,
            data,
            redis_pool,
            thread_pool,
            incoming: format!("{}::incoming", name),
            processing: format!("{}::processing", name),
        }
    }

    fn process(
        handle: fn(T, U),
        bincode: Vec<u8>,
        queue: String,
        data: U,
        redis_pool: Pool<RedisConnectionManager>,
        thread_pool: ThreadPool,
    )
    {
        thread_pool.execute(move || {
            if let Ok(decoded) = bincode::deserialize(&bincode[..]) {
                handle(decoded, data);
                if let Ok(redis) = redis_pool.get() {
                    let _finished: Vec<u8> =
                        redis.lrem(queue, 0, &bincode[..]).expect("removing task from queue");
                } else {
                    error!("getting connection from pool");
                }
            } else {
                error!("deserializing to bincode");
            }
        });
    }

    pub fn listen(self, handle: fn(T, U))
    {
        loop {
            if let Ok(redis) = self.redis_pool.get() {
                let bincode: Vec<u8> = redis
                    .brpoplpush(&self.incoming, &self.processing, 0)
                    .expect("pushing task into incoming queue");

                TaskQueue::process(
                    handle,
                    bincode,
                    self.processing.clone(),
                    self.data.clone(),
                    self.redis_pool.clone(),
                    self.thread_pool.clone(),
                );
            } else {
                error!("getting connection from pool");
            }
        }
    }
}

#[derive(Clone)]
pub struct TaskQueueClient<T>
{
    marker: PhantomData<T>,
    redis_pool: Pool<RedisConnectionManager>,
    incoming: String,
    processing: String,
}

impl<T> TaskQueueClient<T>
where
    T: 'static + Serialize + DeserializeOwned + Debug,
{
    pub fn new(
        name: &str,
        redis_pool: Pool<RedisConnectionManager>,
    ) -> TaskQueueClient<T>
    {
        TaskQueueClient {
            marker: PhantomData::<T>,
            redis_pool,
            incoming: format!("{}::incoming", name),
            processing: format!("{}::processing", name),
        }
    }


    pub fn send(&self, value: &T) -> RedisResult<()>
    {
        let encoded: Vec<u8> = bincode::serialize(value)
            .expect("serializing value to bincode");

        let redis = self.redis_pool.get()
            .expect("getting connection from pool");

        redis.lpush(&self.incoming, encoded)?;
        Ok(())
    }
}
