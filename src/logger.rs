/// AStAPrint-Common - Logger.rs
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use chrono::Local;

use log::{set_boxed_logger,
          set_max_level,
          Level,
          LevelFilter,
          Log,
          Metadata,
          Record,
          SetLoggerError};

pub struct Logger
{
    name: String,
}

impl Log for Logger
{
    fn enabled(&self, metadata: &Metadata) -> bool
    {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record)
    {
        // if record.level() == Level::Error {
        // panic!(
        // "{} {} {} {}",
        // Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
        // self.name,
        // record.level(),
        // record.args()
        // );
        // }

        if self.enabled(record.metadata()) {
            println!(
                "{} {} {} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
                record.level(),
                self.name,
                record.args()
            );
        }
    }

    fn flush(&self)
    {

    }
}

impl Logger
{
    pub fn init(name: &str) -> Result<(), SetLoggerError>
    {
        let logger = Logger {
            name: name.to_string(),
        };

        set_boxed_logger(Box::new(logger))?;

        set_max_level(LevelFilter::Debug);

        Ok(())
    }
}
