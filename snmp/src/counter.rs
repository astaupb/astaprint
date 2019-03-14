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

impl Sub for CounterValues
{
    type Output = CounterValues;

    fn sub(self, other: CounterValues) -> CounterValues
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

impl Default for CounterValues
{
    fn default() -> CounterValues
    {
        CounterValues {
            total: -1,
            copy_total: -1,
            copy_bw: -1,
            print_total: -1,
            print_bw: -1,
        }
    }
}
