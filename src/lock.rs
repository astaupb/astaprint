/// AStAPrint - lock.rs
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
use std::{fs::{metadata,
               rename},
          process::Command,
          thread,
          time};

fn touch_file(path_to_file: &str) -> bool
{
    Command::new("touch").arg(path_to_file).status().expect(&format!("touching {}", path_to_file)).success()
}

#[derive(Debug)]

pub struct Lock
{
    pub lock: String,
    pub idle: String,
}

impl Lock
{
    pub fn new(base: &str) -> Lock
    {
        let lock = format!("{}.lock", base);

        let idle = format!("{}.idle", base);

        if metadata(&lock).is_err() && metadata(&idle).is_err() {
            touch_file(&idle);
        }

        Lock { lock,
               idle, }
    }

    pub fn is_grabbed(&self) -> bool
    {
        metadata(&self.lock).is_ok()
    }

    pub fn grab(&self)
    {
        loop {
            if rename(&self.idle, &self.lock).is_ok() {
                break;
            } else {
                thread::sleep(time::Duration::from_millis(42));
            }
        }
    }

    pub fn release(&self) -> bool
    {
        metadata(&self.lock).is_ok() && rename(&self.lock, &self.idle).is_ok()
    }
}

impl Drop for Lock
{
    fn drop(&mut self)
    {
        assert!(self.release());
    }
}
