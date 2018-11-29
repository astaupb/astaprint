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

use diesel::{
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
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
            decrypt_pdf,
            pdfjam,
        },
    },
    task::DispatcherTask,
    uid::UID,
};

pub fn dispatch(mut task: DispatcherTask, pool: Pool<ConnectionManager<MysqlConnection>>)
{
    let uid = UID::from(task.uid);

    let mut pdf_document = PDFDocument::new(&task.data[..], &task.info.password);

    // swap filename with pdf title if filename is empty
    if task.info.filename == "" {
        task.info.filename = pdf_document.title.clone().unwrap_or(String::from("Anonymous"));
    }

    if task.info.password != "" {
        task.data = decrypt_pdf(task.data, &task.info.password).expect("decrypting pdf");;
        pdf_document = PDFDocument::new(&task.data[..], "");
    }

    task.info.pagecount = pdf_document.get_pagecount();

    let page_info = pdf_document.get_pageinfo();

    if page_info.size != Valid(PageSize::A3) && page_info.size != Valid(PageSize::A4) {
        info!("invalid pageformat, using pdfjam to fix it");

        task.data = pdfjam(task.data, &page_info).expect("jamming pdf to valid format");

        pdf_document = PDFDocument::new(&task.data, "");
    }

    if !task.info.color {
        task.data = create_greyscale_pdf(task.data).expect("creating greyscale pdf with gs");

        pdf_document = PDFDocument::new(&task.data, "");
    }

    let connection = pool.get().expect("getting mysql connection from pool");

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

    info!("{:x} finished", uid);
}
