pub mod worker;
pub mod dispatcher;

use redis::queue::Unique;

impl Unique for worker::WorkerTask
{
    fn uid(&self) -> Vec<u8>
    {
        self.uid.clone()
    }
    fn hex_uid(&self) -> String
    {
        hex::encode(&self.uid[..])
    }
}

impl Unique for dispatcher::DispatcherTask
{
    fn uid(&self) -> Vec<u8>
    {
        self.uid.clone()
    }
    fn hex_uid(&self) -> String
    {
        hex::encode(&self.uid[..])
    }
}