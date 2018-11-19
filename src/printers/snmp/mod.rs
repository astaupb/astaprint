pub mod counter;
pub mod session;

use diesel::prelude::*;

use self::counter::CounterOids;
use printers::{
    *,
};
use establish_connection;

#[derive(Debug, Clone)]

pub struct PrinterInterface
{
    pub ip: String,
    pub community: String,
    pub counter: CounterOids,
    pub queue_ctl: QueueControl,
    pub energy_ctl: EnergyControl,
}

impl PrinterInterface
{
    pub fn from_device_id(device_id: u16) -> PrinterInterface
    {
        let (row, queue_ctl, energy_ctl, community, ip): (
            Counter,
            QueueCtl,
            EnergyCtl,
            String,
            String,
        ) = select_printer_interface_information(device_id);

        PrinterInterface {
            ip,
            community,
            counter: CounterOids {
                total: vec_from_oid_str(&row.total),
                print_black: vec_from_oid_str(&row.print_black),
                print_color: row.print_color.map(|s| vec_from_oid_str(&s)),
                copy_black: vec_from_oid_str(&row.copy_black),
                copy_color: row.copy_color.map(|s| vec_from_oid_str(&s)),
            },
            queue_ctl: QueueControl::from(queue_ctl),
            energy_ctl: EnergyControl::from(energy_ctl),
        }
    }
}

#[derive(Debug, Clone)]

pub struct QueueControl
{
    pub oid: Vec<u64>,
    pub cancel: i32,
    pub clear: i32,
}

impl From<QueueCtl> for QueueControl
{
    fn from(queue_ctl: QueueCtl) -> Self
    {
        QueueControl {
            oid: vec_from_oid_str(&queue_ctl.oid),
            cancel: queue_ctl.cancel,
            clear: queue_ctl.clear,
        }
    }
}

#[derive(Debug, Clone)]

pub struct EnergyControl
{
    pub oid: Vec<u64>,
    pub wake: i32,
    pub sleep: i32,
}

impl From<EnergyCtl> for EnergyControl
{
    fn from(energy_ctl: EnergyCtl) -> Self
    {
        EnergyControl {
            oid: vec_from_oid_str(&energy_ctl.oid),
            wake: energy_ctl.wake,
            sleep: energy_ctl.sleep,
        }
    }
}

fn vec_from_oid_str(oid: &str) -> Vec<u64>
{
    use std::str::FromStr;

    oid.split(".").map(|x| u64::from_str(x).expect("converting oid str to u64")).collect()
}

pub fn select_printer_interface_information(
    device_id: u16,
) -> (Counter, QueueCtl, EnergyCtl, String, String)
{
    let result: (Counter, QueueCtl, EnergyCtl, String, String) = printer_counter::table
        .inner_join(
            printer_model::table
                .inner_join(printers::table)
                .inner_join(printer_energy_ctl::table)
                .inner_join(printer_queue_ctl::table),
        )
        .select((
            printer_counter::all_columns,
            printer_queue_ctl::all_columns,
            printer_energy_ctl::all_columns,
            printers::community,
            printers::ip,
        ))
        .filter(printers::device_id.eq(device_id))
        .first(&establish_connection())
        .expect("fetching printer interface information");

    result
}

pub fn select_device_ids() -> Vec<u16>
{
    printers::table.select(printers::device_id).load(&establish_connection()).expect("fetching device ids")
}
