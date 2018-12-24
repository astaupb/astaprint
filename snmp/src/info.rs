use std::ffi::CString;
use crate::parse_oid;
use mysql::printers::PrinterInfo;

#[derive(Debug, Clone)]
pub struct InfoOids
{
    pub model: Vec<u64>,
    pub hostname: Vec<u64>,
    pub location: Vec<u64>,
    pub mac: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct InfoValues
{
    pub model: String,
    pub hostname: String,
    pub location: String,
    pub mac: String,
}

impl InfoOids
{
    pub fn to_oid_vec(&self) -> Vec<&Vec<u64>>
    {
        let mut oids: Vec<&Vec<u64>> = Vec::with_capacity(4); 

        oids.push(&self.model);
        oids.push(&self.hostname);
        oids.push(&self.location);
        oids.push(&self.mac);

        oids
    }
}

impl From<Vec<CString>> for InfoValues
{
    fn from(values: Vec<CString>) -> InfoValues
    {
        InfoValues {
            model: values[0].clone().into_string().unwrap(),
            hostname: values[1].clone().into_string().unwrap(),
            location: values[2].clone().into_string().unwrap(),
            mac: values[3].clone().into_string().unwrap(),
        } 
    }
}

impl<'a> From<&'a PrinterInfo> for InfoOids
{
    fn from(info: &'a PrinterInfo) -> InfoOids
    {
        InfoOids {
            model: parse_oid(&info.model),
            hostname: parse_oid(&info.hostname),
            location: parse_oid(&info.location),
            mac: parse_oid(&info.mac),
        }
    }
}

