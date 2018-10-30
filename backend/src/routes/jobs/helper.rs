use astaprint::job::data::JobOptions;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]

pub enum Value
{
    S(String),
    I(u16),
    B(bool),
}

#[derive(Serialize, Deserialize, Debug)]

pub struct JobOptionsUpdate
{
    pub duplex: Option<u8>,
    pub copies: Option<u16>,
    pub collate: Option<bool>,
    pub keep: Option<bool>,
    pub a3: Option<bool>,
    pub nup: Option<u8>,
    pub nuppageorder: Option<u8>,
    pub range: Option<String>,
}

pub trait Update
{
    fn merge(self, update: JobOptionsUpdate) -> Self;
}

impl Update for JobOptions
{
    fn merge(mut self, update: JobOptionsUpdate) -> JobOptions
    {
        if let Some(duplex) = update.duplex {
            self.duplex = duplex;
        }

        if let Some(copies) = update.copies {
            self.copies = copies;
        }

        if let Some(collate) = update.collate {
            self.collate = collate;
        }

        if let Some(keep) = update.keep {
            self.keep = keep;
        }

        if let Some(a3) = update.a3 {
            self.a3 = a3;
        }

        if let Some(nup) = update.nup {
            self.nup = nup;
        }

        if let Some(nuppageorder) = update.nuppageorder {
            self.nuppageorder = nuppageorder;
        }

        if let Some(range) = update.range {
            self.range = range;
        }

        self
    }
}

pub trait Hex
{
    fn to_hex(&self) -> String;
}

impl Hex for [u8]
{
    fn to_hex(&self) -> String
    {
        use std::fmt::Write;

        let mut hex = String::new();

        for byte in self {
            write!(hex, "{:02x}", byte);
        }

        hex
    }
}

#[derive(FromForm, Debug)]

pub struct UploadForm
{
    pub filename: Option<String>,
    pub password: Option<String>,
    pub color: Option<bool>,
}
