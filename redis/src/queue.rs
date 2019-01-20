use r2d2_redis::{
    r2d2::Pool,
    redis::{
        Value,
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

pub trait Unique
{
    fn uid(&self) -> Vec<u8>;
    fn hex_uid(&self) -> String;
}

#[derive(Clone)]
pub struct TaskQueue<T, D, C>
{
    redis_pool: Pool<RedisConnectionManager>,
    thread_pool: ThreadPool,
    data: D,
    task: PhantomData<T>,
    command: PhantomData<C>,
    incoming: String,
    processing: String,
}

impl<T, D, C> TaskQueue<T, D, C>
where
    T: 'static + Serialize + DeserializeOwned + Debug,
    D: 'static + Send + Clone,
    C: 'static + Send + Clone,
{
    pub fn new(
        name: &str,
        data: D,
        redis_pool: Pool<RedisConnectionManager>,
        thread_pool: ThreadPool,
    ) -> TaskQueue<T, D, C>
    {
        TaskQueue {
            task: PhantomData::<T>,
            command: PhantomData::<C>,
            data,
            redis_pool,
            thread_pool,
            incoming: format!("{}::incoming", name),
            processing: format!("{}::processing", name),
        }
    }

    fn process(
        &self,
        handle: fn(T, D, TaskQueueClient<T, C>),
        bincode: Vec<u8>,
    )
    {
            if let Ok(decoded) = bincode::deserialize(&bincode[..]) {
                handle(decoded, self.data.clone(), TaskQueueClient::from(self));
                if let Ok(redis) = self.redis_pool.get() {
                    let _removed: Value = redis.lrem(&self.processing, 0, bincode).expect("removing task from queue");
                } else {
                    error!("getting connection from pool");
                }
            } else {
                error!("deserializing to bincode");
            }
    }

    pub fn listen(self, handle: fn(T, D, TaskQueueClient<T, C>))
    {
        loop {
            if let Ok(redis) = self.redis_pool.get() {
                let bincode: Vec<u8> = redis
                    .brpoplpush(&self.incoming, &self.processing, 0)
                    .expect("pushing task into incoming queue");

                self.process(
                    handle,
                    bincode,
                );
            } else {
                error!("getting connection from pool");
            }
        }
    }
}

#[derive(Clone)]
pub struct TaskQueueClient<T, C>
{
    task: PhantomData<T>,
    command: PhantomData<C>,
    redis_pool: Pool<RedisConnectionManager>,
    incoming: String,
    processing: String,
}

impl<'a, T, D, C> From<&'a TaskQueue<T, D, C>> for TaskQueueClient<T, C>
{
    fn from(queue: &TaskQueue<T, D, C>) -> TaskQueueClient<T, C>
    {
        TaskQueueClient {
            task: queue.task,
            command: queue.command,
            redis_pool: queue.redis_pool.clone(),
            incoming: queue.incoming.clone(),
            processing: queue.processing.clone(),
        }
    }
}

impl<T, C> TaskQueueClient<T, C>
where
    T: 'static + Serialize + DeserializeOwned + Unique + Debug,
    C: 'static + Serialize + DeserializeOwned + Debug + Send + Clone,
{
    pub fn new(
        name: &str,
        redis_pool: Pool<RedisConnectionManager>,
    ) -> TaskQueueClient<T, C>
    {
        TaskQueueClient {
            task: PhantomData::<T>,
            command: PhantomData::<C>,
            redis_pool,
            incoming: format!("{}::incoming", name),
            processing: format!("{}::processing", name),
        }
    }

    pub fn get_processing(&self) -> Vec<T>
    {
        let redis = self.redis_pool.get()
            .expect("getting connection from pool");

        let processing: Vec<Vec<u8>> = redis.lrange(&self.processing, 0, -1)
            .expect("getting processing list");

        processing.iter().map(|binary| {
            bincode::deserialize(&binary[..])
                .expect("deserialing processing task from bincode")
        }).collect()
    }
        
    pub fn get_incoming(&self) -> Vec<T>
    {
        let redis = self.redis_pool.get()
            .expect("getting connection from pool");

        let incoming: Vec<Vec<u8>> = redis.lrange(&self.incoming, 0, -1)
            .expect("getting incoming list");

        incoming.iter().map(|binary| {
            bincode::deserialize(&binary[..])
                .expect("deserialing incoming task from bincode")
        }).collect()
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

#[derive(Clone)]
pub struct CommandClient<T, C>
{
    task: PhantomData<T>,
    command: PhantomData<C>,
    redis_pool: Pool<RedisConnectionManager>,
    queue: String,
}

impl<'a, T, C> From<(&'a TaskQueueClient<T, C>, &'a str)> for CommandClient<T, C>
{
    fn from((client, uid): (&TaskQueueClient<T, C>, &str)) -> CommandClient<T, C>
    {
        CommandClient {
            task: client.task,
            command: client.command,
            redis_pool: client.redis_pool.clone(),
            queue: format!("{}::{}", client.processing, uid),
        }
    }
}

impl<T, C> CommandClient<T, C>
where
    T: 'static + Serialize + DeserializeOwned + Unique + Debug,
    C: 'static + Serialize + DeserializeOwned + Debug + Send + Clone,
{
    pub fn send_command(&self, command: &C) -> RedisResult<()>
    {
        let encoded: Vec<u8> = bincode::serialize(command)
            .expect("serializing command to bincode");
        
        let redis = self.redis_pool.get()
            .expect("gettig connection from pool");

        redis.lpush(&self.queue, encoded)?;

        Ok(())
    }

    pub fn receive_command(&self) -> RedisResult<Option<C>>
    {
        let redis = self.redis_pool.get()
            .expect("getting redis connection from pool");

        redis.rpop(&self.queue)
            .map(|value: Value|
                if let Value::Data(binary) = value {
                    Some(bincode::deserialize(&binary[..])
                        .expect("deserializing Command"))
                } else {
                    None
                }
            )
    }
}
