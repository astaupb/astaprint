/// AStAPrint PDF - subprocesses.rs
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
use jobs::{
    pdf::pageinfo::{
        Is::Almost,
        PageInfo,
        PageOrientation,
        PageSize,
    },
    tmp::TemporaryFile,
};

use std::{
    fs::rename,
    io,
    process::{
        Command,
        Stdio,
    },
};

pub fn decrypt_pdf(data: Vec<u8>, password: &str) -> io::Result<Vec<u8>>
{
    let path = TemporaryFile::create(data)?;
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.decr", path);

    Command::new("qpdf")
        .args(&["--decrypt", &format!("--password={}", password), &path, &outfile])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    rename(&outfile, &path)?;

    Ok(TemporaryFile::remove(&path)?)
}

pub fn pdfjam(data: Vec<u8>, info: &PageInfo) -> io::Result<Vec<u8>>
{
    let path = TemporaryFile::create(data)?;

    let mut arguments = ["--a4paper", "--no-landscape", "--checkfiles", "--outfile", &path, &path];

    if info.size == Almost(PageSize::A3) {
        arguments[0] = "--a3paper";
    }

    if info.orientation == PageOrientation::Landscape {
        arguments[1] = "--landscape";
    }

    Command::new("pdfjam").args(&arguments).stdout(Stdio::piped()).stderr(Stdio::piped()).status()?;

    Ok(TemporaryFile::remove(&path)?)
}

pub fn create_greyscale_pdf(data: Vec<u8>) -> io::Result<Vec<u8>>
{
    let path = TemporaryFile::create(data)?;
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.grey", path);

    let arguments: [&str; 9] = [
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        "-sDEVICE=pdfwrite",
        "-dCompabilityLevel=1.4",
        "-sColorConversionStrategy=Gray",
        "-dProcessColorModel=/DeviceGray",
        &format!("-sOutputFile={}", outfile),
        &path,
    ];

    let gs_pdf =
        Command::new("gs").args(&arguments).stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

    for line in String::from_utf8_lossy(&gs_pdf.stdout).lines() {
        if line.contains("Substituting") || line.contains("Loading") {
            warn!("{:?}", line);
            continue;
        }
    }

    rename(&outfile, &path)?;

    Ok(TemporaryFile::remove(&path)?)
}
