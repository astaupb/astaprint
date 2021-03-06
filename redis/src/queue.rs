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

use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::mpsc,
    thread,
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
    ) -> TaskQueue<T, D, C>
    {
        TaskQueue {
            task: PhantomData::<T>,
            command: PhantomData::<C>,
            data,
            redis_pool,
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
            } else {
                error!("deserializing to bincode");
            }
    }

    pub fn listen(self, handle: fn(T, D, TaskQueueClient<T, C>))
    {
        loop {
            if let Ok(mut redis) = self.redis_pool.get() {
                let bincode: Vec<u8> = redis
                    .brpoplpush(&self.incoming, &self.processing, 0)
                    .expect("pushing task into incoming queue");

                let _: i32 = redis.expire(&self.processing, 72)
                    .expect("setting expiration of processing key");

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

    pub fn refresh_timeout(&self) -> RedisResult<u32>
    {
        let mut redis = self.redis_pool.get()
            .expect("getting connection from pool");

        redis.expire(&self.processing, 72)?;
        redis.expire(&self.incoming, 72)
    }

    pub fn get_processing(&self) -> Vec<T>
    {
        let mut redis = self.redis_pool.get()
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
        let mut redis = self.redis_pool.get()
            .expect("getting connection from pool");

        let incoming: Vec<Vec<u8>> = redis.lrange(&self.incoming, 0, -1)
            .expect("getting incoming list");

        incoming.iter().map(|binary| {
            bincode::deserialize(&binary[..])
                .expect("deserialing incoming task from bincode")
        }).collect()
    }

    pub fn remove(&self, uid: Vec<u8>) -> RedisResult<u32>
    {
        let mut redis = self.redis_pool.get()
            .expect("getting connection from pool");

        redis.lrem(&self.incoming, 0, uid)
    }

    pub fn finish(&self, value: &T) -> RedisResult<()>
    {
        let encoded: Vec<u8> = bincode::serialize(value)
            .expect("serializing value to bincode");

        let mut redis = self.redis_pool.get()
            .expect("getting connection from pool");

        redis.lrem(&self.processing, 0, encoded)?;

        Ok(())
    }
    pub fn send(&self, value: &T) -> RedisResult<()>
    {
        let encoded: Vec<u8> = bincode::serialize(value)
            .expect("serializing value to bincode");

        let mut redis = self.redis_pool.get()
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
        
        let mut redis = self.redis_pool.get()
            .expect("gettig connection from pool");

        redis.lpush(&self.queue, encoded)?;

        Ok(())
    }

    pub fn receive_command(&self) -> RedisResult<Option<C>>
    {
        let mut redis = self.redis_pool.get()
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

    pub fn get_command_receiver(self) -> mpsc::Receiver<C> {
        let (sender, receiver) = mpsc::channel::<C>();
        {
            let redis_pool = self.redis_pool.clone();
            let queue = self.queue;
            thread::spawn(move || {
                let mut redis = redis_pool.get()
                    .expect("getting connection from pool");
                loop {
                    match redis.brpop::<&str, Vec<Vec<u8>>>(&queue, 72) {
                        Ok(bulk) => {
                            if bulk.is_empty() {
                                debug!("brpop timeout");
                                return;
                            } else {
                                let command: C = bincode::deserialize(&bulk[1][..])
                                    .expect("deserializing Command");

                                if let Err(e) = sender.send(command) {
                                    error!("sending command: {:?}", e);
                                    return;
                                }
                            }
                        },
                        Err(e) => {
                            error!("brpop: {:?}", e);
                            return;
                        }
                    }

                }
            });
        }
        receiver
    }
}
