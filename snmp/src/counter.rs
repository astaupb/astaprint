use std::ops::Sub;
use crate::parse_oid;
use mysql::printers::PrinterCounter;

#[derive(Debug, Clone)]
pub struct CounterOids
{
    pub total: Vec<u64>,
    pub copy_total: Vec<u64>,
    pub copy_bw: Vec<u64>,
    pub print_total: Vec<u64>,
    pub print_bw: Vec<u64>,
}

impl<'a> From<&'a PrinterCounter> for CounterOids
{
    fn from(counter: &'a PrinterCounter) -> CounterOids
    {
        CounterOids {
            total: parse_oid(&counter.total),    
            copy_total: parse_oid(&counter.copy_total),    
            copy_bw: parse_oid(&counter.copy_bw),    
            print_total: parse_oid(&counter.print_total),    
            print_bw: parse_oid(&counter.print_bw),
        }
    }
}

impl CounterOids
{
    pub fn to_oid_vec(&self) -> Vec<&Vec<u64>>
    {
        let mut oids: Vec<&Vec<u64>> = Vec::with_capacity(5); 

        oids.push(&self.total);
        oids.push(&self.copy_total);
        oids.push(&self.copy_bw);
        oids.push(&self.print_total);
        oids.push(&self.print_bw);

        oids
    }
}

#[derive(Debug, Clone)]
pub struct CounterValues
{
    pub total: i64,
    pub copy_total: i64,
    pub copy_bw: i64,
    pub print_total: i64,
    pub print_bw: i64,
}

impl From<Vec<i64>> for CounterValues
{
    fn from(values: Vec<i64>) -> CounterValues
    {
        CounterValues {
            total: values[0],
            copy_total: values[1],
            copy_bw: values[2],
            print_total: values[3],
            print_bw: values[4],
        } 
    }
}
/*

impl Sub for CounterValues
{
    type Output = CounterValues;

    fn sub(self, other: CounterValues) -> CounterValues
    {
        let total = self.total - other.total;

        let print_black = self.print_black - other.print_black;

        let copy_black = self.copy_black - other.copy_black;

        let print_color = match (self.print_color, other.print_color) {
            (Some(some), Some(other)) => Some(some - other),
            _ => None,
        };

        let copy_color = match (self.copy_color, other.copy_color) {
            (Some(some), Some(other)) => Some(some - other),
            _ => None,
        };

        CounterValues {
            total,
            print_black,
            copy_black,
            print_color,
            copy_color,
        }
    }
}
*/
