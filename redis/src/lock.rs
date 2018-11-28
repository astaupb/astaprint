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
use r2d2_redis::redis::{
    self,
    Client,
    Connection,
    RedisResult,
    Value,
};

use std::{
    env,
    thread,
    time,
};

use astacrypto::random_bytes;

pub struct Lock
{
    user_id: u32,
    connection: Connection,
    value: Vec<u8>,
}

impl Lock
{
    pub fn new(user_id: u32) -> Lock
    {
        let url = env::var("ASTAPRINT_REDIS_URL").expect("reading redis url from environment");
        let client = Client::open(&url[..]).expect("creating redis client");
        let connection = client.get_connection().expect("getting redis connection from client");

        let value = random_bytes(20);

        Lock {
            user_id,
            connection,
            value,
        }
    }

    pub fn is_grabbed(&self) -> bool
    {
        let result: RedisResult<Vec<u8>> = redis::cmd("GET").arg(self.user_id).query(&self.connection);
        debug!("{:?}", result);
        result.is_ok() && result.unwrap().len() > 0
    }

    pub fn grab(&self)
    {
        loop {
            if let Ok(Value::Okay) = redis::cmd("SET")
                .arg(self.user_id)
                .arg(self.value.clone())
                .arg("NX")
                .arg("PX")
                .arg(420000)
                .query(&self.connection)
            {
                break;
            } else {
                thread::sleep(time::Duration::from_millis(42));
            }
        }
    }

    pub fn release(&self) -> bool
    {
        // check if value is the own to avoid removing a lock created by another client
        redis::cmd("GET").arg(self.user_id).query(&self.connection) == Ok(self.value.clone())
            && redis::cmd("DEL").arg(self.user_id).query(&self.connection) == Ok(Value::Int(1))
    }
}

impl Drop for Lock
{
    fn drop(&mut self)
    {
        assert!(self.release());
    }
}
