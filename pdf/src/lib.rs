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
        ghostscript_colored_pagecount,
        pdfjam,
    },
};

#[derive(Debug, Clone)]
pub struct DispatchResult
{
    pub pdf: Vec<u8>,
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

    let title = pdf_document.title.clone().unwrap_or_else(|| String::from(""));

    let pagecount = pdf_document.pagecount();

    let mut pageinfo = pdf_document.get_pageinfo();

    info!("{:?}: {:?}", pageinfo, pdf_document.pagesizes());
    if pageinfo.size != Valid(PageSize::A3) && pageinfo.size != Valid(PageSize::A4) {

        pdf = pdfjam(pdf, &pageinfo).expect("jamming pdf to valid format");

        pdf_document = PDFDocument::new(&pdf[..], "");

        pageinfo = pdf_document.get_pageinfo();
        info!("pdfjam {:?}: {:?}", pageinfo, pdf_document.pagesizes());
    }

    let a3 = match pageinfo.size {
        Valid(PageSize::A3) => true,
        Valid(PageSize::A4) => false,
        _ => panic!("pdfjam does not work"),
    };

    let colored = ghostscript_colored_pagecount(&pdf[..], pdf_document.pagecount()).expect("running ghostscript");

    let preview_0 = pdf_document.render_preview(0).unwrap();
    let preview_1 = pdf_document.render_preview(1);
    let preview_2 = pdf_document.render_preview(2);
    let preview_3 = pdf_document.render_preview(3);

    DispatchResult {
        pdf,
        preview_0,
        preview_1,
        preview_2,
        preview_3,
        title,
        a3,
        pagecount,
        colored,
    }
}
