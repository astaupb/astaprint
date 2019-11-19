use crate::{
    schema::*,
    printers::*,
};
use diesel::{
    prelude::*,
    update,
};

pub fn update_printer(printer: Printer, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(printers::table.filter(printers::device_id.eq(printer.device_id)))
        .set((
            printers::hostname.eq(printer.hostname),
            printers::ip.eq(printer.ip),
            printers::community.eq(printer.community),
            printers::mac.eq(printer.mac),
            printers::location.eq(printer.location),
            printers::has_a3.eq(printer.has_a3),
            printers::coin_operated.eq(printer.coin_operated),
            printers::description.eq(printer.description),
            printers::watch_toner.eq(printer.watch_toner),
            printers::watch_tray1.eq(printer.watch_tray1),
            printers::watch_tray2.eq(printer.watch_tray2),
            printers::watch_tray3.eq(printer.watch_tray3),
            printers::watch_interval.eq(printer.watch_interval),
            printers::last_watched.eq(printer.last_watched),
        ))
        .execute(connection)
}
