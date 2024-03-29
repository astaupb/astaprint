pub mod info;
pub mod options;
use chrono::NaiveDateTime;

use self::{
    info::JobInfo,
    options::{
        JobOptions,
        pagerange::PageRange,
    },
};

use crate::ppd::PPD;

/// representation of the print job including all important information except the file itself
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job
{
    pub id: u32,
    pub info: JobInfo,
    pub options: JobOptions,
    pub timestamp: i64,
    pub created: i64,
    pub updated: i64,
}

impl From<(u32, Vec<u8>, Vec<u8>, NaiveDateTime, NaiveDateTime)> for Job
{
    fn from(row: (u32, Vec<u8>, Vec<u8>, NaiveDateTime, NaiveDateTime)) -> Job
    {
        Job {
            id: row.0,
            info: JobInfo::from(&row.1[..]),
            options: JobOptions::from(&row.2[..]),
            timestamp: row.3.timestamp(),
            created: row.3.timestamp(),
            updated: row.4.timestamp(),
        }
    }
}

impl<'a> From<(u32, &'a [u8], &'a [u8], NaiveDateTime, NaiveDateTime)> for Job
{
    fn from((id, info, options, created, updated): (u32, &'a [u8], &'a [u8], NaiveDateTime, NaiveDateTime)) -> Job
    {
        Job {
            id,
            info: JobInfo::from(info),
            options: JobOptions::from(options),
            timestamp: created.timestamp(),
            created: created.timestamp(),
            updated: updated.timestamp(),
        }
    }
}

impl Job
{
    /// calculates the number of pages which are printed depending on the options which are set
    pub fn pages_to_print(&self) -> u16
    {
        let range = PageRange::new(&self.options.range, self.info.pagecount as usize).expect("valid PageRange");
        let mut count = range.pagecount();

        count = (count / usize::from(self.options.nup))
            + match count % usize::from(self.options.nup) {
                0 => 0,
                _ => 1,
            };

        if self.options.a3 {
            count *= 2;
        }

        count as u16 * self.options.copies
    }

    /// estimation of how many sheets of paper where saved compared to using the default options
    pub fn score(&self) -> i16
    {
        let range = PageRange::new(&self.options.range, self.info.pagecount as usize).expect("valid PageRange");

        let max_pages = range.pagecount() as u16 * self.options.copies;

        let mut paper_to_print = self.pages_to_print();

        if self.options.duplex > 0 {
            paper_to_print = (paper_to_print / 2) + (paper_to_print % 2);
        }

        debug!("max_pages: {}, paper_to_print: {}", max_pages, paper_to_print);
        max_pages as i16 - paper_to_print as i16
    }

    /// translates the print job into printer job language (PJL) using the PPD options and the file itself
    pub fn translate_for_printer(&mut self, mut ppd: PPD, mut data: Vec<u8>) -> Vec<u8>
    {
        let mut header: Vec<u8> = Vec::with_capacity(8096);
        header.append(&mut ppd.begin);

        if self.options.a3 {
            header.append(&mut ppd.page_size_a3);
            header.append(&mut ppd.page_region_a3);
        } else {
            header.append(&mut ppd.page_size_a4);
            header.append(&mut ppd.page_region_a4);
        }

        match self.options.duplex {
            0 => {
                header.append(&mut ppd.duplex_off);
            },
            1 => {
                header.append(&mut ppd.duplex_long);
            },
            2 => {
                header.append(&mut ppd.duplex_short);
            },
            _ => (),
        }

        if self.options.copies > 1 {
            let pjl_string = String::from_utf8_lossy(if self.options.collate {
                &ppd.copies
            } else {
                &ppd.copies_collate
            });
            header.append(&mut
                pjl_string.replace("&copies;", &format!("{}", self.options.copies)).as_bytes().to_owned()
            );
        }

        if self.options.color {
            header.append(&mut ppd.color);
        } else {
            header.append(&mut ppd.greyscale);
        }
        if self.options.bypass {
            header.append(&mut ppd.tray_bypass);
        } else {
            header.append(&mut ppd.tray_auto);
        }

        header.append(&mut ppd.to_pdf);

        header.append(&mut data);

        header.append(&mut ppd.end);

        header
    }

}
