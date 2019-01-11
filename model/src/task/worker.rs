use crate::job::options::JobOptions;
use sodium::random_bytes;

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