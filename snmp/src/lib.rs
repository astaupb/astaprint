#[macro_use]
extern crate serde_derive;
pub mod tool;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CounterValues
{
    pub total: i64,
    pub copy_total: i64,
    pub copy_bw: i64,
    pub print_total: i64,
    pub print_bw: i64,
}

impl Default for CounterValues
{
    fn default() -> CounterValues
    {
        CounterValues{
            total: -1,
            copy_total: -1,
            copy_bw: -1,
            print_total: -1,
            print_bw: -1,
        } 
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StatusValues
{
    pub scan: i64,
    pub copy: i64,
    pub toner_c: i64,
    pub toner_m: i64,
    pub toner_y: i64,
    pub toner_k: i64,
    pub tray_1: i64,
    pub tray_2: i64,
    pub tray_3: i64,
}

impl StatusValues
{
    pub fn ok(&self) -> bool
    {
        self.scan == 0
            && self.copy == 0
            && self.toner_c > 0
            && self.toner_m > 0
            && self.toner_y > 0
            && self.toner_k > 0
            && self.tray_1 > 0 || self.tray_3 > 0 // A4
            && self.tray_2 > 0 // A3
    }
}

impl Default for StatusValues
{
    fn default() -> StatusValues
    {
        StatusValues{
            scan: -1,
            copy: -1,
            toner_c: -1,
            toner_m: -1,
            toner_y: -1,
            toner_k: -1,
            tray_1: -1,
            tray_2: -1,
            tray_3: -1,
        } 
    }
}

use std::ops::Sub;

impl Sub for CounterValues
{
    type Output = CounterValues;

    fn sub(
        self,
        other: CounterValues,
    ) -> CounterValues
    {
        CounterValues {
            total: self.total - other.total,
            copy_total: self.copy_total - other.copy_total,
            copy_bw: self.copy_bw - other.copy_bw,
            print_total: self.print_total - other.print_total,
            print_bw: self.print_bw - other.print_bw,
        }
    }
}
