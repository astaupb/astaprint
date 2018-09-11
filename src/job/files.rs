/// AStAPrint-Common - Files.rs
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

use std::{env, fs::remove_file};

#[derive(Serialize, Deserialize)]
pub struct JobFiles {
    pub pdf: String,
    pub tmp: String,
    pub index: String,
    pub preview: Vec<String>,
}

impl JobFiles {
    pub fn new(uid: &str, username: &str) -> JobFiles {
        let spooldir = env::var("ASTAPRINT_SPOOL_DIR").expect("reading spooldir from environemt");
        let userdir = format!("{}/user/{}", spooldir, username);
        let mut preview: Vec<String> = Vec::new();
        for i in 0..4 {
            preview.push(format!("{}/preview/{}-{}", userdir, uid, i));
        }
        JobFiles {
            pdf: format!("{}/pdf/{}", userdir, uid),
            tmp: format!("{}/tmp/{}", userdir, uid),
            index: format!("{}/index/{}", userdir, uid),
            preview: preview,
        }
    }
    pub fn clean_up(&self, pagecount: &u16) {
        remove_file(&self.pdf).expect("removing pdf file");
        remove_file(&self.index).expect("removing index file");
        for (i, file) in self.preview.iter().enumerate() {
            if i < *pagecount as usize {
                remove_file(&file).expect("removing preview file");
            }
        }
    }
}
