use crate::job::options::JobOptions;
use sodium::random_bytes;

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use r2d2_redis::RedisConnectionManager;

use snmp::PrinterInterface;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkerTask
{
    pub user_id: u32,
    pub uid: Vec<u8>,
}

impl WorkerTask
{
    pub fn new(user_id: u32) -> WorkerTask
    {
        WorkerTask {
            user_id, uid: random_bytes(20),
        }
    }
}

#[derive(Clone)]
pub struct WorkerState
{
    pub printer_interface: PrinterInterface,
    pub mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    pub redis_pool: Pool<RedisConnectionManager>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum WorkerCommand
{
    Print(u32),
    Cancel,
    Hungup,
}
