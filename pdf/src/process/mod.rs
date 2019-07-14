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
pub mod child;

use crate::{
    pageinfo::{
        Is::{
            Almost,
            Valid,
        },
        PageOrientation,
        PageSize,
        PageSummary,
    },
    process::child::{
        pdfjam,
        gs_inkcov,
        qpdf_decrypt,
        qpdf_rotate,
        qpdf_pages,
        qpdf_force_version,
    },
    tmp::TmpFile,
};

use std::{
    fs::{
        rename, remove_file,
    },
    io,
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

pub fn decrypt_pdf_from_data(
    data: Vec<u8>,
    password: &str,
) -> Result<Vec<u8>, DecryptionError>
{
    let path = TmpFile::create(&data[..])?;

    decrypt_pdf(&path, password)?;

    Ok(TmpFile::remove(&path)?)
}

pub fn decrypt_pdf(
    path: &str,
    password: &str,
) -> Result<(), DecryptionError>
{
    let out = &format!("{}.decr", path);

    if qpdf_decrypt(path, out, password)?.wait()?.success()
    {
        rename(out, path)?;

        Ok(())
    }
    else {
        remove_file(path)?;

        Err(DecryptionError::PasswordError)
    }
}

pub fn pdfnup(path: &str, nup: u8, nuppageorder: u8, a3: bool, landscape: bool) -> io::Result<()>
{
    let mut arguments = ["--a4paper", "--nup", "1x1", "--no-landscape", "--reflect", "false", "--checkfiles", "--outfile", path, path];

    if a3 {
        arguments[0] = "--a3paper";
    }

    if landscape || nup == 2 {
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

    pdfjam(&arguments)?.wait()?;

    Ok(())

}

pub fn force_page_size(
    path: &str,
    info: &PageSummary
) -> io::Result<()>
{
    let mut arguments = ["--a4paper", "--no-landscape", "--checkfiles", "--outfile", &path, &path];

    if info.size == Almost(PageSize::A3) {
        arguments[0] = "--a3paper";
    }

    // assuming only valid orientations here
    if info.orientation() == Valid(PageOrientation::Landscape) {
        arguments[1] = "--landscape";
    }

    pdfjam(&arguments)?.wait()?;

    Ok(())
}

pub fn force_pdf_version(path: &str) -> io::Result<()>
{
    let out = &format!("{}_out", path);

    qpdf_force_version(path, out)?
        .wait()?;

    remove_file(path)?;

    rename(out, path)?;

    Ok(())
}

pub fn trim_pages(path: &str, pagerange: &str) -> io::Result<()>
{

    let out = &format!("{}_out", path);

    qpdf_pages(path, out, pagerange)?
        .wait()?;

    remove_file(path)?;

    rename(out, path)?;
    
    Ok(())
}

pub fn rotate_pages(path: &str, landscape: bool, pagerange: &str) -> io::Result<()>
{
    let out = &format!("{}_out", path);

    let angle = if landscape {"-90"} else {"+90"};

    debug!("rotating");
    qpdf_rotate(path, out, angle, pagerange)?
        .wait()?;

    debug!("out: {:?}", out);
    rename(out, path)?;

    Ok(())
}

pub fn colored_pagecount(
    path: &str,
    pagecount: u32,
) -> io::Result<u32>
{
    let gs = gs_inkcov(path)?
        .wait_with_output()?;

    let mut non_colored = 0;

    for line in String::from_utf8_lossy(&gs.stdout[..]).lines() {
        if line.ends_with("CMYK OK") && line.starts_with(" 0.00000  0.00000  0.00000  ") {
            non_colored += 1;
            debug!("non_colored: {}", non_colored);
        }
    }

    Ok(pagecount - non_colored)
}
