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
pub mod soffice;

use crate::{
    pageinfo::{
        Is::Almost,
        PageInfo,
        PageOrientation,
        PageSize,
    },
    tmp::TmpFile,
};

use std::{
    fs::rename,
    io,
    process::{
        Command,
        Child,
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
    fn from(error: io::Error) -> DecryptionError
    {
        DecryptionError::IoError(error) 
    }
}
pub fn decrypt_pdf(data: Vec<u8>, password: &str) -> Result<Vec<u8>, DecryptionError>
{
    let path = TmpFile::create(&data[..])?;
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.decr", path);

    if Command::new("qpdf")
        .args(&[
            "--decrypt",
            &format!("--password={}", password),
            &path,
            &outfile,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?.success() {
        
        rename(&outfile, &path)?;

        Ok(TmpFile::remove(&path)?)
    } else {
        Err(DecryptionError::PasswordError) 
    }
}

pub fn pdfjam(data: Vec<u8>, info: &PageInfo) -> io::Result<Vec<u8>>
{
    let path = TmpFile::create(&data[..])?;

    let mut arguments = [
        "--a4paper",
        "--no-landscape",
        "--checkfiles",
        "--outfile",
        &path,
        &path,
    ];

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

pub fn ghostscript_pdfwrite_bw(output: &str, input: &str) -> Child
{
    Command::new("gs")
        .args(&[
            "-dSAFER",
            "-dBATCH",
            "-dNOPAUSE",
            "-sDEVICE=pdfwrite",
            "-dCompabilityLevel=1.4",
            "-sColorConversionStrategy=Gray",
            "-dProcessColorModel=/DeviceGray",
            &format!("-sOutputFile={}", output),
            &input,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().expect("executing gs")
}

pub fn ghostscript_inkcov(input: &str) -> Child
{
    Command::new("gs")
        .args(&[
            "-dSAFER",
            "-dBATCH",
            "-dNOPAUSE",
            "-sDEVICE=inkcov",
            "-o", "-",
            &input,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().expect("executing gs")

}

pub fn ghostscript(data: &[u8]) -> io::Result<(Vec<u8>, u32)>
{
    let path = TmpFile::create(&data[..])?;
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.grey", path);

    // speculating that inkvoc is faster than pdfwrite
    let gs_pdfbw = ghostscript_pdfwrite_bw(&outfile, &path);

    let gs_inkcov = ghostscript_inkcov(&path)
        .wait_with_output()
        .expect("waiting for gs_inkcov");

    let mut non_colored = 0;
    for line in String::from_utf8_lossy(&gs_inkcov.stdout[..]).lines() {
        if line.ends_with("CMYK OK")
        && line.starts_with(" 0.00000  0.00000  0.00000  ") {
            non_colored += 1;
            debug!("non_colored: {}", non_colored);
        }
    }
    let _gs_pdfbw = gs_pdfbw
        .wait_with_output()
        .expect("waiting for gs_pdfwrite_bw"); 

    rename(&outfile, &path)?;

    Ok((TmpFile::remove(&path)?, non_colored))
}
