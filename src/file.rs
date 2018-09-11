/// AStAPrint-Common - File.rs
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

use std::process::{Command, Stdio};

#[derive(Debug, PartialEq)]
pub enum FileType {
    PDF,
    PostScript,
    Invalid,
}

pub fn get_file_type(path_to_file: &str) -> FileType {
    let file = Command::new("file")
        .arg(path_to_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawning file")
        .wait_with_output()
        .expect("waiting for file output");

    if file.status.success() {
        for line in String::from_utf8_lossy(&file.stdout).lines() {
            if line.contains("PostScript document text") {
                return FileType::PostScript;
            } else if line.contains("PDF document") {
                return FileType::PDF;
            }
        }
    }
    return FileType::Invalid;
}
