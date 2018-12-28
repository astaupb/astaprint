/// AStAPrint - Jobs - PDF
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
#[macro_use]
extern crate log;

pub mod document;
pub mod pageinfo;
pub mod subprocesses;
pub mod tmp;

use crate::{
    document::PDFDocument,
    pageinfo::{
        Is::Valid,
        PageSize,
    },
    subprocesses::{
        ghostscript,
        pdfjam,
    },
};


#[derive(Debug, Clone)]
pub struct DispatchResult
{
    pub pdf: Vec<u8>,
    pub pdf_bw: Vec<u8>,
    pub preview_0: Vec<u8>,
    pub preview_1: Option<Vec<u8>>,
    pub preview_2: Option<Vec<u8>>,
    pub preview_3: Option<Vec<u8>>,
    pub title: String,
    pub a3: bool,
    pub pagecount: u32,
    pub colored: u32,
}

pub fn sanitize(mut pdf: Vec<u8>) -> DispatchResult
{
    let mut pdf_document = PDFDocument::new(&pdf[..], "");

    let title = pdf_document.title.clone()
        .unwrap_or(String::from(""));

    let pagecount = pdf_document.get_pagecount();

    let mut page_info = pdf_document.get_pageinfo();

    if page_info.size != Valid(PageSize::A3) && page_info.size != Valid(PageSize::A4) {
        debug!("{:?} needs pdfjam", page_info);

        pdf = pdfjam(pdf, &page_info).expect("jamming pdf to valid format");

        pdf_document = PDFDocument::new(&pdf[..], "");

        page_info = pdf_document.get_pageinfo();
    }

    let a3 = match page_info.size {
        Valid(PageSize::A3) => true,
        Valid(PageSize::A4) => false,
        _ => panic!("pdfjam does not work"),
    };

    let (pdf_bw, colored) = ghostscript(&pdf[..])
        .expect("running ghostscript");

    let preview_0 = pdf_document.render_preview(0).unwrap();
    let preview_1 = pdf_document.render_preview(1);
    let preview_2 = pdf_document.render_preview(2);
    let preview_3 = pdf_document.render_preview(3);

    DispatchResult {
        pdf, pdf_bw, preview_0, preview_1, preview_2, preview_3, title, a3, pagecount, colored,
    }
}

#[cfg(test)]
mod tests
{
    use std::fs::{File, read};
    use std::io::Read;
    use crate::dispatch;
    #[test]
    fn dispatch_test_pdf()
    {
        let mut data = read("test.pdf").unwrap();

        let result = dispatch(data, "filename", "", true);
        println!("pagecount: {}, colored: {}", result.pagecount, result.colored);

    }
}
