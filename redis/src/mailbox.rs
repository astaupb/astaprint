/// AStAPrint
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
    marker::PhantomData,
    fmt::Debug,
};

#[derive(Debug, Clone)]
pub struct Mailbox<T>
{
    pub uid: Vec<u8>,
    pool: Pool<RedisConnectionManager>,
    marker: PhantomData<T>,
}

impl<T> From<(Vec<u8>, Pool<RedisConnectionManager>)> for Mailbox<T>
{
    fn from((uid, pool): (Vec<u8>, Pool<RedisConnectionManager>)) -> Mailbox<T>
    {
        Mailbox {
            uid, pool, marker: PhantomData::<T>,
        } 
    }
}

impl<T> Mailbox<T>
where
    T: Serialize + DeserializeOwned + Debug
{
    pub fn try_receive(&self) -> Option<T>
    {
        let redis = self.pool.get()
            .expect("getting connection from pool");

        let result: RedisResult<Value> = redis.get(self.uid.clone());
        if let Ok(Value::Data(data)) = result {
            let message: T = bincode::deserialize(&data[..])
                 .expect("deserializing message from bincode");
            let _result: Value = redis.del(data)
                .expect("removing message from mailbox");
            Some(message)
        } else {
            None 
        }
    }
    pub fn send(&self, message: &T)
    {
        let data = bincode::serialize(message)
            .expect("serializing message to bincode");

        let redis = self.pool.get()
            .expect("getting connection from pool");

        let _value: Value = redis.set_ex(self.uid.clone(), data, 720)
            .expect("sending message to mailbox");
    }
}

#[cfg(test)]
mod tests
{
    use crate::create_redis_pool;
    use std::env;
    use astacrypto::random_bytes;
    use super::Mailbox;
    #[test]
    fn message_passing()
    {
        let url = env::var("ASTAPRINT_REDIS_URL").unwrap();
        let redis_pool = create_redis_pool(&url, 3);
        let mailbox: Mailbox<String> = Mailbox::from((random_bytes(20), redis_pool)); 
        let result = mailbox.try_receive();
        println!("{:?}", result);
        assert!(result.is_none());
        mailbox.send(&String::from("hello"));
        let result = mailbox.try_receive();
        println!("{:?}", result);
        assert!(result.is_some());

    }
}

