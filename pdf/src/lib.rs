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
pub mod process;
pub mod tmp;

use crate::{
    document::DocumentInfo,
    pageinfo::{
        Is::{
            Valid,
            Almost,
        },
        PageSize,
        PageOrientation,
    },
    tmp::TmpFile,
    process::{
        colored_pagecount,
        rotate_pages,
        force_pdf_version,
        force_page_size,
        preprocess,
        image_preprocess,
    },
};

use model::job::options::pagerange::PageRange;

#[derive(Debug, Clone)]
pub struct SanitizeResult
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
    pub landscape: bool,
}

/// function to sanitize pdf files
/// 1. rotate the pages so all are the same direction
/// 2. convert to a3 or a4 if neccessary
/// 3. convert to pdf version 1.4 if version is higher
/// 4. preprocess as requested with the arg "do_preprocess"
///     * 1 = using ghostscript
///     * 2 = using conversion to image and back (sledgehammer method)
///     * else do nothing (only recommended when processing scans)
/// 5. count number of colored pages and render previews
pub fn sanitize_pdf(data: Vec<u8>, uid: &str, do_preprocess: u8) -> SanitizeResult
{
    let path = &TmpFile::create(&data[..])
        .expect("creating tmp file");

    let mut info = DocumentInfo::new(path);

    let title = info.title.clone().unwrap_or_else(|| String::from(""));

    let pagecount = info.pagecount();

    let mut pageinfo = info.get_page_summary();

    debug!("{} {:?}: {:?}", uid, pageinfo, info.pagesizes());

    let orientation = pageinfo.orientation();

    if orientation != Valid(PageOrientation::Landscape)
        && orientation != Valid(PageOrientation::Portrait)
    {
        let pages = PageRange::from_list(
            pageinfo.pages.iter().map(|page| {
                if orientation == Almost(PageOrientation::Portrait) {
                    page == &PageOrientation::Portrait
                } else {
                    page == &PageOrientation::Landscape
                }
            }).collect()
        );

        let mut range = format!("{}", pages);
        range.pop(); //remove trailing comma
        debug!("{} {:?}, rotating: {}", uid, orientation, range);

        rotate_pages(path, orientation == Almost(PageOrientation::Landscape), &range)
            .expect("rotating pages");

        info = DocumentInfo::new(path);

        pageinfo = info.get_page_summary();
    }

    let landscape = match pageinfo.orientation() {
        Valid(PageOrientation::Landscape) => true,
        Valid(PageOrientation::Portrait) => false,
        Almost(PageOrientation::Landscape) => {
            error!("{} qpdf rotate does not work", uid);
            true
        },
        Almost(PageOrientation::Portrait) => {
            error!("{} qpdf rotate does not work", uid);
            false
        },
    };

    if pageinfo.size != Valid(PageSize::A3)
        && pageinfo.size != Valid(PageSize::A4)
    {
        force_page_size(path, &pageinfo).expect("jamming pdf to valid format");

        info = DocumentInfo::new(path);

        pageinfo = info.get_page_summary();
        debug!("{} pdfjam {:?}: {:?}", uid, pageinfo, info.pagesizes());
    }

    let a3 = match pageinfo.size {
        Valid(PageSize::A3) => true,
        Valid(PageSize::A4) => false,
        Almost(PageSize::A3) => {
            error!("{} pdfjam does not work", uid);
            true
        },
        Almost(PageSize::A4) => {
            error!("{} pdfjam does not work", uid);
            false
        },
    };

    let version = info.get_minor_version();

    debug!("{} PDF minor version: {}", uid, version);

    if version > 4 {
        debug!("{} version 1.{} > 1.4, forcing 1.4", uid, version);
        force_pdf_version(path)
            .expect("converting pdf");

        info = DocumentInfo::new(path);

        assert!(info.get_minor_version() < 5);
    }

    match do_preprocess {
        1 =>
            preprocess(path)
                .expect("preprocessing pdf"),
        2 =>
            image_preprocess(path, pagecount)
                .expect("image_preprocessing pdf"),
        _ => (),
    }

    let colored = colored_pagecount(path, info.pagecount()).expect("running ghostscript");

    let preview_0 = info.render_preview(0).unwrap();
    let preview_1 = info.render_preview(1);
    let preview_2 = info.render_preview(2);
    let preview_3 = info.render_preview(3);

    SanitizeResult {
        pdf: TmpFile::remove(path)
            .expect("removing tmpfile"),
        preview_0,
        preview_1,
        preview_2,
        preview_3,
        title,
        a3,
        pagecount,
        colored,
        landscape,
    }
}
