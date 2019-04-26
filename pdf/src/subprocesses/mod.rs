/// AStAPrint PDF - subprocesses.rs
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use crate::{
    pageinfo::{
        Is::Almost,
        PageInfo,
        PageOrientation,
        PageSize,
    },
    tmp::TmpFile,
};

use model::job::options::pagerange::PageRange;

use std::{
    fs::{
        rename,
    },
    io,
    process::{
        Child,
        Command,
        Stdio,
    },
};

#[derive(Debug)]
pub enum DecryptionError
{
    IoError(io::Error),
    PasswordError,
}

impl From<io::Error> for DecryptionError
{
    fn from(error: io::Error) -> DecryptionError { DecryptionError::IoError(error) }
}
pub fn decrypt_pdf(
    data: Vec<u8>,
    password: &str,
) -> Result<Vec<u8>, DecryptionError>
{
    let path = TmpFile::create(&data[..])?;
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.decr", path);

    if Command::new("qpdf")
        .args(&["--decrypt", &format!("--password={}", password), &path, &outfile])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?
        .success()
    {
        rename(&outfile, &path)?;

        Ok(TmpFile::remove(&path)?)
    }
    else {
        Err(DecryptionError::PasswordError)
    }
}
pub fn pdfnup(data: Vec<u8>, nup: u8, nuppageorder: u8, a3: bool, landscape: bool) -> io::Result<Vec<u8>>
{
    let path = TmpFile::create(&data[..])?;

    let mut arguments = ["--a4paper", "--nup", "1x1", "--no-landscape", "--reflect", "false", "--checkfiles", "--outfile", &path, &path];

    if a3 {
        arguments[0] = "--a3paper";
    }

    if landscape {
        arguments[3] = "--landscape";
    }

    if nuppageorder > 0 {
        arguments[5] = "true";
    }

    arguments[2] = match nup {
        1 => "1x1",
        2 => "2x1",
        4 => "2x2",
        _ => "1x1",
    };

    Command::new("pdfjam")
        .args(&arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    Ok(TmpFile::remove(&path)?)

}

pub fn pdfjam(
    data: Vec<u8>,
    info: &PageInfo,
) -> io::Result<Vec<u8>>
{
    let path = TmpFile::create(&data[..])?;

    let mut arguments = ["--a4paper", "--no-landscape", "--checkfiles", "--outfile", &path, &path];

    if info.size == Almost(PageSize::A3) {
        arguments[0] = "--a3paper";
    }

    if info.orientation == PageOrientation::Landscape {
        arguments[1] = "--landscape";
    }

    Command::new("pdfjam")
        .args(&arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    Ok(TmpFile::remove(&path)?)
}
pub fn ghostscript_pdfwrite_trim(
    output: &str,
    input: &str,
    first_page: u32,
    last_page: u32,
    a3: bool,
) -> Child
{
    Command::new("gs")
        .args(&[
            "-dSAFER", "-dBATCH",
            "-dNOPAUSE",
            "-dFIXEDMEDIA",
            "-dUseTrimBox",
            &format!("-dFirstPage={}", first_page),
            &format!("-dLastPage={}", last_page),
            &format!(
                "-sPAPERSIZE={}",
                if a3 {
                    "a3"
                }
                else {
                    "a4"
                }
            ),
            "-sDEVICE=pdfwrite",
            "-dCompabilityLevel=1.6",
            "-dPrinted",
            &format!("-sOutputFile={}", output),
            &input,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("executing gs")
}

pub fn pdfunite(input: &[String], output: &str) -> Child
{
    Command::new("pdfunite")
        .args(input)
        .arg(output)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("executing pdf")
}

pub fn trim_pdf(input: Vec<u8>, pagerange: &PageRange, a3: bool) -> Vec<u8>
{
    let input = TmpFile::create(&input[..]).expect("creating TmpFile");
    let mut files: Vec<String> = Vec::new();

    let ranges = pagerange.ranges.clone();
    for range in ranges {
        let output = format!("{}_{}", input, range.minuend);
        let _gs = ghostscript_pdfwrite_trim(&output, &input, range.subtrahend, range.minuend, a3)
            .wait_with_output().expect("executing gs");
        files.push(output);
    }

    let output = format!("{}_out", input);

    let _pdfunite = pdfunite(&files, &output)
        .wait_with_output().expect("executing pdfunite");

    TmpFile::remove(&output).expect("removing TmpFile")
}

pub fn ghostscript_inkcov(input: &str) -> Child
{
    Command::new("gs")
        .args(&["-dSAFER", "-dBATCH", "-dNOPAUSE", "-sDEVICE=inkcov", "-o", "-", &input])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("executing gs")
}

pub fn ghostscript_colored_pagecount(
    data: &[u8],
    pagecount: u32,
) -> io::Result<u32>
{
    let path = TmpFile::create(&data[..])?;

    let gs_inkcov = ghostscript_inkcov(&path).wait_with_output().expect("waiting for gs_inkcov");

    let mut non_colored = 0;
    for line in String::from_utf8_lossy(&gs_inkcov.stdout[..]).lines() {
        if line.ends_with("CMYK OK") && line.starts_with(" 0.00000  0.00000  0.00000  ") {
            non_colored += 1;
            debug!("non_colored: {}", non_colored);
        }
    }
    Ok(pagecount - non_colored)
}
