use crate::parse_oid;
use mysql::printers::PrinterControl;

#[derive(Debug, Clone)]
pub struct ControlOids
{
    pub queue: Vec<u64>,
    pub cancel: i32,
    pub clear: i32,
    pub energy: Vec<u64>,
    pub wake: i32,
    pub sleep: i32,
}

impl<'a> From<&'a PrinterControl> for ControlOids
{
    fn from(control: &PrinterControl) -> ControlOids
    {
        ControlOids {
            queue: parse_oid(&control.queue),
            cancel: control.cancel,
            clear: control.clear,
            energy: parse_oid(&control.energy),
            wake: control.wake,
            sleep: control.sleep,
        }
    }
}
