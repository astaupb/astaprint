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
    pub job_id: u32,
    pub user_id: u32,
    pub uid: Vec<u8>,
    pub options: JobOptions,
}

impl WorkerTask
{
    pub fn new(id: u32, user_id: u32) -> WorkerTask
    {
        let uid = random_bytes(20);
        let options = JobOptions::default();
        WorkerTask {
            job_id: id,
            user_id,
            uid,
            options,
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

#[derive(PartialEq, Debug)]
pub enum WorkerCommand
{
    Print,
    Cancel,
}

