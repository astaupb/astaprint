use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};
use redis::lock::Lock;

#[derive(Debug)]
pub struct JournalLock
{
    lock: Lock,
}

impl From<Pool<RedisConnectionManager>> for JournalLock
{
    fn from(pool: Pool<RedisConnectionManager>) -> JournalLock
    {
        let mut lock = Lock::new("journal", pool);
        lock.grab();
        JournalLock {
            lock,
        }
    }
}
