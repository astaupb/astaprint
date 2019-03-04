pub mod pagerange;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JobOptions
{
    pub color: bool,
    pub duplex: u8,
    pub copies: u16,
    pub collate: bool,
    pub keep: bool,
    pub a3: bool,
    pub nup: u8,
    pub nuppageorder: u8,
    pub range: String,
}

impl JobOptions
{
    pub fn serialize(&self) -> Vec<u8>
    {
        bincode::serialize(&self).expect("serializing JobOptions")
    }
}

impl Default for JobOptions
{
    fn default() -> JobOptions
    {
        JobOptions {
            color: true,
            duplex: 0,
            copies: 1,
            collate: false,
            keep: false,
            a3: false,
            nup: 1,
            nuppageorder: 0,
            range: String::from(""),
        }
    }
}
