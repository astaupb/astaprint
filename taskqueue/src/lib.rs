#[macro_use]
extern crate log;
extern crate bincode;
extern crate r2d2_redis;
extern crate serde;
use r2d2_redis::{
    r2d2::Pool,
    redis::{
        Client,
        Commands,
        Connection,
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
};

pub fn connect(url: &str) -> RedisResult<Connection>
{
    let client = Client::open(&url[..])?;
    Ok(client.get_connection()?)
}

pub fn create_pool(url: &str) -> Pool<RedisConnectionManager>
{
    Pool::builder()
        .build(RedisConnectionManager::new(url).expect("creating Connection Manager"))
        .expect("creating Connection Pool")
}

#[derive(Clone, Debug)]
pub struct TaskQueue<T>
{
    pool: Pool<RedisConnectionManager>,
    marker: PhantomData<T>,
    name: String,
    incoming: String,
    processing: String,
}

impl<T> TaskQueue<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    pub fn new(name: &str, pool: Pool<RedisConnectionManager>) -> TaskQueue<T>
    {
        TaskQueue {
            pool,
            marker: PhantomData::<T>,
            name: String::from(name),
            incoming: format!("{}::incoming", name),
            processing: format!("{}::processing", name),
        }
    }

    pub fn listen(self, handle: fn(T)) -> RedisResult<()>
    {
        loop {
            if let Ok(redis) = self.pool.get() {
                let val: Vec<u8> = redis.brpoplpush(&self.incoming, &self.processing, 0)?;

                if let Ok(decoded) = bincode::deserialize(&val[..]) {
                    handle(decoded);
                    redis.lrem(&self.processing, 0, &val[..])?;
                } else {
                    error!("coudl not decode task"); 
                }
            } else {
                error!("could not get connection from pool");
            }
        }
    }

    pub fn send(&self, value: &T) -> RedisResult<()>
    {
        let encoded: Vec<u8> = bincode::serialize(value).expect("serializing value to bincode");

        let redis = self.pool.get().expect("getting connection from pool");

        redis.lpush(&self.incoming, encoded)?;
        Ok(())
    }
}

#[test]
pub fn listener()
{
    use std::{
        env,
        thread,
        time,
    };
    let url = env::var("TEST_REDIS_URL").unwrap();
    let pool = create_pool(&url);
    let pool2 = pool.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(1));
            let sender = TaskQueue::<u32>::new("worker", pool.clone());
            sender.send(&420).unwrap();
        }
    });
    let listener = TaskQueue::<u32>::new("worker", pool2);
    listener.listen(|x| {
        assert_eq!(x, 420);
        println!("{}", x); 
    });
}