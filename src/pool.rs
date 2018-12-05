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
    r2d2::Pool as RedisPool,
    RedisConnectionManager,
};

use diesel::{
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool as MysqlPool,
    },
};

use std::env;

pub fn create_redis_pool(url: &str, max_size: u32) -> RedisPool<RedisConnectionManager>
{
    RedisPool::builder()
        .max_size(max_size)
        .build(RedisConnectionManager::new(url).expect("creating Connection Manager"))
        .expect("creating Redis Connection Pool")
}

pub fn create_mysql_pool(max_size: u32) -> MysqlPool<ConnectionManager<MysqlConnection>>
{
    MysqlPool::builder()
        .max_size(max_size)
        .build(ConnectionManager::<MysqlConnection>::new(
            env::var("ASTAPRINT_DATABASE_URL").expect("getting database url from environment"),
        ))
        .expect("creating Mysql Connection Pool")
}
