pub mod pagerange;

/// options of a print job
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JobOptions
{
    pub color: bool,
    pub duplex: u8,
    pub copies: u16,
    pub collate: bool,
    pub bypass: bool,
    pub keep: bool,
    pub a3: bool,
    pub nup: u8,
    pub nuppageorder: u8,
    pub range: String,
    pub displayname: String,
}

/// serializes the struct into a binary format which is used for storing it in the database
impl JobOptions
{
    pub fn serialize(&self) -> Vec<u8>
    {
        bincode::serialize(&self).expect("serializing JobOptions")
    }
}

/// deserializes the struct back from the binary format
impl<'a> From<&'a [u8]> for JobOptions
{
    fn from(bytes: &'a [u8]) -> JobOptions
    {
        bincode::deserialize(bytes).expect("deserializing JobOptions")
    }
}

/// default values for the options
impl Default for JobOptions
{
    fn default() -> JobOptions
    {
        JobOptions {
            color: false,
            duplex: 0,
            copies: 1,
            collate: false,
            bypass: false,
            keep: false,
            a3: false,
            nup: 1,
            nuppageorder: 0,
            range: String::from(""),
            displayname: String::from(""),
        }
    }
}
