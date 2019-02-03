// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
