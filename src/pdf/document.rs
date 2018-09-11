/// AStAPrint-Common - Document.rs
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

use poppler::{PopplerDocument, PopplerPage};

use cairo::{enums::Format::ARgb32, Context, ImageSurface};

use std::fs::File;

use pdf::pageinfo::PageInfo;

#[derive(Debug)]
pub struct PDFDocument {
    pages: Vec<PopplerPage>,
    pagesizes: Vec<(f64, f64)>,
    pagecount: usize,
}

impl PDFDocument {
    pub fn new(file: &str, password: &str) -> PDFDocument {
        let data = PopplerDocument::new_from_file(file, password)
            .expect("getting poppler document from path");

        let pagecount = data.get_n_pages();

        let mut pages: Vec<PopplerPage> = Vec::with_capacity(pagecount);

        for i in 0..pagecount {
            pages.push(
                data.get_page(i)
                    .expect("getting page from poppler document"),
            );
        }
        let pagesizes = pages.iter().map(|page| page.get_size()).collect();

        PDFDocument {
            pages,
            pagesizes,
            pagecount,
        }
    }

    pub fn get_pagecount(&self) -> u16 {
        self.pagecount as u16
    }

    fn get_full_pageinfo(&self) -> Vec<PageInfo> {
        self.pagesizes
            .iter()
            .map(|sizes| PageInfo::from_points(sizes.0, sizes.1))
            .collect()
    }

    pub fn get_pageinfo(&self) -> PageInfo {
        PageInfo::from_multiple(self.get_full_pageinfo())
    }

    fn render_page(&self, number: usize, path: &str) {
        let (w, h) = self.pagesizes[number];
        let page = &self.pages[number];
        let mut surface = ImageSurface::create(ARgb32, w as i32, h as i32)
            .expect("creating cairo image surface for preview rendering");

        let mut context = Context::new(&mut surface);

        context.save();
        page.render(&mut context);
        context.restore();
        context.show_page();

        let mut f: File = File::create(path).expect("opening file for writing preview");
        surface
            .write_to_png(&mut f)
            .expect("writing cairo surface to file");
    }

    pub fn render_previews_up_to(&self, number: usize, path: &Vec<String>) {
        for i in (0..number).filter(|n| n < &self.pagecount && n < &path.len()) {
            self.render_page(i, &path[i]);
        }
    }
}
