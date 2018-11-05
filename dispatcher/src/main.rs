/// AStAPrint-Workers - Dispatch.rs
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

extern crate cairo;
extern crate inotify;
extern crate json_receiver;
extern crate poppler;
extern crate serde_json;
extern crate threadpool;

extern crate astaprint;
extern crate dispatcher;

use json_receiver::JsonReceiver;

use threadpool::ThreadPool;

use astaprint::{
    filetype::FileType,
    job::{
        DispatchWorkerJSON,
        Job,
        PrintWorkerJSON,
    },
    logger::Logger,
};
use dispatcher::{
    pdf::{
        document::PDFDocument,
        pageinfo::{
            Is::{
                Almost,
                Valid,
            },
            PageSize,
        },
    },
    subprocesses::{
        create_greyscale_pdf,
        create_pdf_from_ps,
        decrypt_pdf,
        pdfjam,
    },
};
use std::{
    env,
    fs::{
        rename,
        File,
    },
};

fn main()
{
    let spooldir = env::var("ASTAPRINT_SPOOL_DIR").expect("reading spooldir from environemt");

    let receiver = JsonReceiver::<DispatchWorkerJSON>::new(&format!("{}/dispatch", spooldir)).spawn();

    let pool = ThreadPool::new(42);

    Logger::init("dispatch").expect("initializing dispatch logger");

    info!("initialized");

    loop {
        let json = receiver.recv().unwrap();

        work(json, &pool);
    }
}

fn work(json: DispatchWorkerJSON, pool: &ThreadPool)
{
    debug!("{:?}", &json);

    pool.execute(move || {
        let mut job = Job::new(json.dispatch);

        let file_type = FileType::new(&job.files.tmp);

        if file_type == FileType::Invalid {
            error!("received unknown file type");

            panic!();
        } else if file_type == FileType::PostScript {
            if !create_pdf_from_ps(&job.files.tmp) {
                error!("could not create ps from pdf");

                panic!();
            }
        }

        let mut pdf_document = PDFDocument::new(&job.files.tmp, &job.data.info.password);

        // swap filename with pdf title if filename is empty
        if job.data.info.filename == "" {
            job.data.info.filename = pdf_document.title.clone().unwrap_or(String::from("Anonymous"));
        }

        job.data.info.pagecount = pdf_document.get_pagecount();

        if job.data.info.password != "" {
            if !decrypt_pdf(&job.files.tmp, &job.data.info.password) {
                error!("could not decrypt pdf with user password");

                panic!();
            }
        }

        let page_info = pdf_document.get_pageinfo();

        if page_info.size != Valid(PageSize::A3) && page_info.size != Valid(PageSize::A4) {
            info!("invalid pageformat, using pdfjam to fix it");

            if !pdfjam(&job.files.tmp, &page_info) {
                error!("could not jam pdf to a4");

                panic!();
            } else {
                pdf_document = PDFDocument::new(&job.files.tmp, &job.data.info.password);
            }
        }

        job.data.info.a3 =
            (page_info.size == Valid(PageSize::A3)) || (page_info.size == Almost(PageSize::A3));

        if !job.data.info.color {
            job.data.info.pagecount = create_greyscale_pdf(&job.files.tmp);

            if job.data.info.pagecount != pdf_document.get_pagecount() {
                error!(
                    "what the fuck, pdfinfo: {}, gs pagecount: {}",
                    pdf_document.get_pagecount(),
                    job.data.info.pagecount
                );

                panic!();
            }

            if job.data.info.pagecount == 0 {
                error!("could not convert pdf to greyscale");

                panic!();
            }

            pdf_document = PDFDocument::new(&job.files.tmp, &job.data.info.password);
        }

        debug!("{:?}", job.files.preview);

        pdf_document.render_previews_up_to(4, &job.files.preview);

        if rename(&job.files.tmp, &job.files.pdf).is_err() {
            error!("could not move tmp file to output dir");

            panic!();
        }

        let jobfile = File::create(&job.files.index).expect("creating index file");

        let uid = job.data.uid.clone();

        serde_json::to_writer(jobfile, &PrintWorkerJSON::from(job.data))
            .expect("serializing job to index file");

        info!("{} finished", &uid);
    });
}
