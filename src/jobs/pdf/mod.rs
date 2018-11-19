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
pub mod document;
pub mod pageinfo;
pub mod subprocesses;

use std::env;

use diesel::{
    insert_into,
    prelude::*,
};

use jobs::*;

use jobs::{
    options::JobOptions,
    pdf::{
        document::PDFDocument,
        pageinfo::{
            Is::Valid,
            PageSize,
        },
        subprocesses::{
            create_greyscale_pdf,
            create_pdf_from_ps,
            decrypt_pdf,
            pdfjam,
        },
    },
    task::DispatcherTask,
    tmp::TemporaryFile,
};

pub fn dispatch(mut task: DispatcherTask)
{
    let mut pdf_document = PDFDocument::new(&task.data[..], &task.info.password);

    // swap filename with pdf title if filename is empty
    if task.info.filename == "" {
        task.info.filename = pdf_document.title.clone().unwrap_or(String::from("Anonymous"));
    }

    let mut tmp_file: Option<TemporaryFile> = None;

    if task.info.password != "" {
        if tmp_file.is_none() {
            tmp_file = Some(TemporaryFile::new(&task.data[..]));
        }
        let path = tmp_file.clone().unwrap().path;
        if !decrypt_pdf(&path, &task.info.password) {
            error!("could not decrypt pdf with user password");

            panic!();
        } else {
            task.data = tmp_file.clone().unwrap().close();
            pdf_document = PDFDocument::new(&task.data, &task.info.password);
        }
    }

    task.info.pagecount = pdf_document.get_pagecount();

    let page_info = pdf_document.get_pageinfo();

    if page_info.size != Valid(PageSize::A3) && page_info.size != Valid(PageSize::A4) {
        info!("invalid pageformat, using pdfjam to fix it");

        if tmp_file.is_none() {
            tmp_file = Some(TemporaryFile::new(&task.data[..]));
        }
        let path = tmp_file.clone().unwrap().path;

        if !pdfjam(&path, &page_info) {
            error!("could not jam pdf to a4");

            panic!();
        } else {
            task.data = tmp_file.clone().unwrap().close();
            pdf_document = PDFDocument::new(&task.data, &task.info.password);
        }
    }

    if !task.info.color {
        if tmp_file.is_none() {
            tmp_file = Some(TemporaryFile::new(&task.data[..]));
        }
        let path = tmp_file.clone().unwrap().path;
        task.info.pagecount = create_greyscale_pdf(&path);

        if task.info.pagecount != pdf_document.get_pagecount() {
            error!(
                "what the fuck, pdfinfo: {}, gs pagecount: {}",
                pdf_document.get_pagecount(),
                task.info.pagecount
            );

            panic!();
        }

        if task.info.pagecount == 0 {
            error!("could not convert pdf to greyscale");

            panic!();
        }

        task.data = tmp_file.clone().unwrap().close();
        pdf_document = PDFDocument::new(&task.data, &task.info.password);
    }
    // FIXME no connection pooling here
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = MysqlConnection::establish(&url).expect("establishing MysqlConnection");

    insert_into(jobs::table)
        .values((
            jobs::user_id.eq(task.user_id),
            jobs::info.eq(task.info.serialize()),
            jobs::options.eq(JobOptions::default().serialize()),
            jobs::data.eq(task.data),
            jobs::preview_0.eq(pdf_document.render_preview(0).unwrap()),
            jobs::preview_1.eq(pdf_document.render_preview(1)),
            jobs::preview_2.eq(pdf_document.render_preview(2)),
            jobs::preview_3.eq(pdf_document.render_preview(3)),
        ))
        .execute(&connection)
        .expect("inserting new job into database");

    info!("finished");
}
