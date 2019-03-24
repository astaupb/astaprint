#[macro_use]
extern crate serde_derive;
pub mod tool; 

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CounterValues
{
    total: i64,
    copy_total: i64,
    copy_bw: i64,
    print_total: i64,
    print_bw: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StatusValues
{
    scan: i64,
    copy: i64,
    toner: i64,
    tray_1: i64,
    tray_2: i64,
    tray_3: i64,
    tray_4: i64,
    tray_5: i64,
}
