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
use jobs::pdf::pageinfo::{
    Is::Almost,
    PageInfo,
    PageOrientation,
    PageSize,
};

use std::{
    fs::{
        rename,
    },
    process::{
        Command,
        Stdio,
    },
};

pub fn decrypt_pdf(path_to_pdf: &str, password: &str) -> bool
{
    // input and output file can not be the same
    // create tmp outfile and rename afterwards to decrypt in place
    let outfile = format!("{}.encr", path_to_pdf);

    Command::new("qpdf")
        .args(&["--decrypt", &format!("--password={}", password), path_to_pdf, &outfile])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect(&format!("decrypting {}", path_to_pdf));

    rename(&outfile, path_to_pdf).is_ok()
}

pub fn pdfjam(path_to_pdf: &str, info: &PageInfo) -> bool
{
    let mut arguments =
        ["--a4paper", "--no-landscape", "--checkfiles", "--outfile", path_to_pdf, path_to_pdf];

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
        .status()
        .expect(&format!("jamming {} to a4", path_to_pdf))
        .success()
}

pub fn create_pdf_from_ps(path_to_pdf: &str) -> bool
{
    let outfile = format!("{}.tmp", path_to_pdf);

    let ps2pdf = Command::new("ps2pdf14")
        .args(&[path_to_pdf, &outfile])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect(&format!("creating pdf from ps: {}", path_to_pdf));

    ps2pdf.success() && rename(&outfile, path_to_pdf).is_ok()
}

/// returns pagecount of converted pdf, in case something failed 0 will be returned
/// create temporary file and rename afterwards to convert in place
pub fn create_greyscale_pdf(path_to_pdf: &str) -> u16
{
    let outfile = format!("{}.grey", path_to_pdf);

    let arguments: [&str; 9] = [
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        "-sDEVICE=pdfwrite",
        "-dCompabilityLevel=1.4",
        "-sColorConversionStrategy=Gray",
        "-dProcessColorModel=/DeviceGray",
        &format!("-sOutputFile={}", outfile),
        path_to_pdf,
    ];

    let gs_pdf = Command::new("gs")
        .args(&arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("waiting for gs_pdf output");

    let mut pagecount = 0;

    if gs_pdf.status.success() {
        let mut substitutions: Vec<String> = Vec::new();

        for line in String::from_utf8_lossy(&gs_pdf.stdout).lines() {
            if line.contains("Substituting") || line.contains("Loading") {
                substitutions.push(String::from(line));

                continue;
            }

            if line.contains("Page") && !line.contains("LastPage") {
                pagecount += 1;
            }
        }
    }

    if rename(&outfile, &path_to_pdf).is_ok() {
        pagecount
    } else {
        0
    }
}
