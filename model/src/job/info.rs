#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobInfo
{
    pub filename: String,
    pub title: String,
    pub pagecount: u32,
    pub colored: u32,
    pub a3: bool,
    pub landscape: bool,
}

impl JobInfo
{
    pub fn serialize(&self) -> Vec<u8>
    {
        bincode::serialize(&self).expect("serializing JobInfo")
    }
}

impl<'a> From<&'a [u8]> for JobInfo
{
    fn from(bytes: &'a [u8]) -> JobInfo
    {
        bincode::deserialize(bytes).expect("deserializing JobInfo")
    }
}
