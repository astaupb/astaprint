use crate::parse_oid;
use mysql::printers::PrinterStatus;
use std::default::Default;

#[derive(Debug, Clone)]
pub struct StatusOids
{
    //uptime: Vec<u64>,
    pub scan: Vec<u64>,
    pub copy: Vec<u64>,
    pub toner: Vec<u64>,
    pub tray_1: Vec<u64>,
    pub tray_2: Vec<u64>,
    pub tray_3: Vec<u64>,
    pub tray_4: Vec<u64>,
}

impl StatusOids
{
    pub fn to_oid_vec(&self) -> Vec<&Vec<u64>>
    {
        let mut oids: Vec<&Vec<u64>> = Vec::with_capacity(9); 

        //oids.push(&self.uptime);
        oids.push(&self.scan);
        oids.push(&self.copy);
        oids.push(&self.toner);
        oids.push(&self.tray_1);
        oids.push(&self.tray_2);
        oids.push(&self.tray_3);
        oids.push(&self.tray_4);

        oids
    }
}

impl<'a> From<&'a PrinterStatus> for StatusOids
{
    fn from(status: &PrinterStatus) -> StatusOids
    {
        StatusOids {
         //   uptime: parse_oid(&status.uptime),
            scan: parse_oid(&status.scan),
            copy: parse_oid(&status.copy),
            toner: parse_oid(&status.toner),
            tray_1: parse_oid(&status.tray_1),
            tray_2: parse_oid(&status.tray_2),
            tray_3: parse_oid(&status.tray_3),
            tray_4: parse_oid(&status.tray_4),
        }
    }
}

#[derive(Debug, Clone)]

pub struct StatusValues
{
    pub scan: i64,
    pub copy: i64,
    pub toner: i64,
    pub tray_1: i64,
    pub tray_2: i64,
    pub tray_3: i64,
    pub tray_4: i64,
}

impl From<Vec<i64>> for StatusValues
{
    fn from(values: Vec<i64>) -> StatusValues
    {
        StatusValues {
            scan: values[0],
            copy: values[1],
            toner: values[2],
            tray_1: values[3],
            tray_2: values[4],
            tray_3: values[5],
            tray_4: values[6],
        }
    }
}

impl Default for StatusValues
{
    fn default() -> StatusValues
    {
        StatusValues {
            scan: -1,
            copy: -1,
            toner: -1,
            tray_1: -1,
            tray_2: -1,
            tray_3: -1,
            tray_4: -1,
        } 
    }
}
