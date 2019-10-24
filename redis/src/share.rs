/// AStAPrint
/// Copyright (C) 2018, 2019  AStA der Universität Paderborn
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
        RedisError,
    },
    RedisConnectionManager,
};

use sodium::random_bytes;

#[derive(Debug, Clone)]
pub struct Share
{
    pool: Pool<RedisConnectionManager>,
}

#[derive(Debug)]
pub enum RedisShareError
{
    RedisErr(RedisError),
    DecodeErr(base64::DecodeError),
}

impl From<RedisError> for RedisShareError
{
    fn from(err: RedisError) -> RedisShareError
    {
        RedisShareError::RedisErr(err) 
    }
}

impl From<base64::DecodeError> for RedisShareError
{
    fn from(err: base64::DecodeError) -> RedisShareError
    {
        RedisShareError::DecodeErr(err) 
    }
}

impl From<Pool<RedisConnectionManager>> for Share
{
    fn from(pool: Pool<RedisConnectionManager>) -> Share
    {
        Share {
            pool, 
        } 
    }
}

impl Share
{
    pub fn new(pool: Pool<RedisConnectionManager>) -> Share
    {
        Share {
            pool,
        }
    }

    pub fn set(&self, data: u32) -> RedisResult<String>
    {
        let connection = self.pool.get().expect("getting connection from pool");

        let key = random_bytes(42);

        connection.set(key.clone(), data)?;

        Ok(base64::encode_config(&key[..], base64::URL_SAFE))
    }

    pub fn get(&self, key: String) -> Result<u32, RedisShareError>
    {
        let key = base64::decode_config(&key[..], base64::URL_SAFE)?;

        let connection = self.pool.get().expect("getting connection from pool");

        let value: u32 = connection.get(key.clone())?;

        connection.del(key)?;
        
        Ok(value)
    }
}
