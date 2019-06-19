/// AStAPrint PDF - document.rs
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
use poppler::{
    PopplerDocument,
    PopplerPage,
};

use cairo::{
    Format,
    Context,
    ImageSurface,
};

use crate::pageinfo::{
    PageInfo, PageSummary,
};

#[derive(Debug)]
pub struct DocumentInfo
{
    pub title: Option<String>,
    pages: Vec<PopplerPage>,
    pagesizes: Vec<(f64, f64)>,
    pagecount: usize,
    version: String,
}

impl DocumentInfo
{
    pub fn new(
        path: &str,
    ) -> DocumentInfo
    {
        let data = PopplerDocument::new_from_file(path, "")
            .expect("PopplerDoucment from data");

        let title = data.get_title();

        let pagecount = data.get_n_pages();

        let mut pages: Vec<PopplerPage> = Vec::with_capacity(pagecount);

        for i in 0 .. pagecount {
            pages.push(data.get_page(i).expect("getting page from poppler document"));
        }

        let pagesizes = pages.iter().map(PopplerPage::get_size).collect();

        let version = data.get_pdf_version_string()
            .expect("valid PDF version string");

        DocumentInfo {
            title,
            pages,
            pagesizes,
            pagecount,
            version,
        }
    }

    pub fn pagecount(&self) -> u32 { self.pagecount as u32 }

    pub fn pagesizes(&self) -> Vec<(f64, f64)> { self.pagesizes.clone() }

    pub fn get_page_summary(&self) -> PageSummary
    {
        PageSummary::from_info(self.pagesizes.iter().map(|sizes| PageInfo::from_points(sizes.0, sizes.1)).collect())
    }

    pub fn get_minor_version(&self) -> u32
    {
        let split: Vec<&str> = self.version.split('.').collect();

        if split.len() > 1 {
            split[1].parse::<u32>().expect("parsing minor version")
        } else {
            error!("unable to parse: {:?}", split);
            5
        }
    }

    pub fn render_preview(
        &self,
        number: usize,
    ) -> Option<Vec<u8>>
    {
        if number >= self.pagecount {
            return None
        }
        let (w, h) = self.pagesizes[number];

        let page = &self.pages[number];

        let surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32)
            .expect("creating cairo image surface for preview rendering");

        let mut context = Context::new(&surface);

        context.save();

        page.render(&mut context);

        context.restore();

        context.show_page();

        let mut png: Vec<u8> = Vec::new();

        surface.write_to_png(&mut png).expect("writing cairo surface to file");

        Some(png)
    }
}
