/// AStAPrin - Jobs - DispatcherTask
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
/// 
use diesel::{
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use threadpool::ThreadPool;

use redis::store::Store;

#[derive(Clone)]
pub struct DispatcherState
{
    pub mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    pub redis_store: Store,
    pub thread_pool: ThreadPool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DispatcherTask
{
    pub user_id: u32,
    pub filename: String,
    pub preprocess: u8,
    pub keep: Option<bool>,
    pub color: Option<bool>,
    pub a3: Option<bool>,
    pub duplex: Option<u8>,
    pub copies: Option<u16>,
    pub uid: Vec<u8>,
}
