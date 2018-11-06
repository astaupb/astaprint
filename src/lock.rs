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
use redis::{Client, Connection, RedisResult, Value};

use std::{
    fs::{
        File,
    },
    env,
    io::Read,
    thread,
    time,
};

pub fn urandom(buf: &mut [u8])
{
    let mut file = File::open("/dev/urandom").expect("opening /dev/urandom");

    file.read_exact(buf).expect("reading /dev/urandom to buffer");
}

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
        let url = env::var("ASTAPRINT_REDIS_URL")
            .expect("reading redis url from environment");
        let client = Client::open(&url[..])
            .expect("creating redis client");
        let connection = client.get_connection()
            .expect("getting redis connection from client");

        let mut value: Vec<u8> = Vec::with_capacity(20);
        urandom(&mut value);

        Lock {
            user_id, connection, value,
        }
    }

    pub fn is_grabbed(&self) -> bool
    {
        let result: RedisResult<Vec<u8>> = redis::cmd("GET").arg(self.user_id).query(&self.connection);
        result.is_ok()
    }

    pub fn grab(&self)
    {
        loop {
            if let Ok(Value::Okay) = redis::cmd("SET").arg(self.user_id).arg(self.value.clone())
                .arg("NX").arg("PX").arg(420000).query(&self.connection)
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
