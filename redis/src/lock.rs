/// AStAPrint - lock.rs
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
    redis::{
        Commands,
        RedisResult,
        Value,
    },
    r2d2::{
        Pool,
    },
    RedisConnectionManager,
};

use std::{
    thread,
    time,
};

use sodium::random_bytes;

#[derive(Debug)]
pub struct Lock
{
    name: String,
    pool: Pool<RedisConnectionManager>,
    value: Option<Vec<u8>>,
}

impl Lock
{
    pub fn new(name: &str, pool: Pool<RedisConnectionManager>) -> Lock
    {
        Lock {
            name: String::from(name),
            pool,
            value: None,
        }
    }

    pub fn is_grabbed(&self) -> bool
    {
        let redis = self.pool.get()
            .expect("getting redis from pool");

        let result: RedisResult<Vec<u8>> = redis.get(&self.name);
        result.is_ok() && result.unwrap().is_empty()
    }

    pub fn grab(&mut self)
    {
        let redis = self.pool.get()
            .expect("getting redis from pool");

        assert!(self.value.is_none());
        self.value = Some(random_bytes(20));

        let mut count = 0;
        loop {
            let result = redis.set_nx(
                &self.name,
                self.value.clone().unwrap()
            );
            if let Ok(Value::Int(1)) = result
            {
                debug!("{} locked after {} tries", self.name, count);
                break;
            } else {
                count += 1;
                thread::sleep(time::Duration::from_millis(42));
            }
        }
    }

    pub fn release(&self) -> bool
    {
        let redis = self.pool.get()
            .expect("getting redis from pool");
        // check if value is the own to avoid removing a lock created by another client
        let _get: RedisResult<Value> = redis.get(&self.name);
        let del: RedisResult<Value> = redis.del(&self.name);

        del == Ok(Value::Int(1))
    }
}
impl Drop for Lock
{
    fn drop(&mut self)
    {
        assert!(self.release());
    }
}
