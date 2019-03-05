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
        Commands,
        RedisResult,
    },
    RedisConnectionManager,
};

use sodium::random_bytes;

#[derive(Debug, Clone)]
pub struct Store
{
    pool: Pool<RedisConnectionManager>,
}

impl From<Pool<RedisConnectionManager>> for Store
{
    fn from(pool: Pool<RedisConnectionManager>) -> Store
    {
        Store {
            pool, 
        } 
    }
}

impl Store
{
    pub fn new(pool: Pool<RedisConnectionManager>) -> Store
    {
        Store {
            pool,
        }
    }

    pub fn set(&self, data: Vec<u8>) -> RedisResult<Vec<u8>>
    {
        let connection = self.pool.get().expect("getting connection from pool");

        let key = random_bytes(20);
        connection.set(key.clone(), data)?;

        debug!("store.set: {:?}", key);
        Ok(key)
    }

    pub fn get(&self, key: Vec<u8>) -> RedisResult<Vec<u8>>
    {
        debug!("store.get: {:?}", key);
        let connection = self.pool.get().expect("getting connection from pool");

        let value = connection.get(key.clone())?;

        connection.del(key)?;
        
        Ok(value)
    }
}
