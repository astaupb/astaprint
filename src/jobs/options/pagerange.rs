// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::{
    fmt,
    str::FromStr,
};

#[derive(Debug)]
struct PageDifference
{
    minuend: u32,
    subtrahend: u32,
}

impl<'a> FromStr for PageDifference
{
    type Err = ();

    fn from_str(difference: &str) -> Result<PageDifference, ()>
    {
        let difference = difference.trim();
        let split: Vec<&str> = difference.split('-').collect();

        if split.len() != 2 {
            return Err(())
        }

        let mut minuend: u32 = match split[0].parse() {
            Ok(int) => int,
            Err(_) => return Err(()),
        };
        let subtrahend: u32 = match split[1].parse() {
            Ok(int) => int,
            Err(_) => return Err(()),
        };

        if minuend > subtrahend {
            minuend = subtrahend;
        }
        Ok(PageDifference {
            minuend,
            subtrahend,
        })
    }
}

#[derive(Debug)]
pub struct PageRange
{
    pages: Vec<bool>,
}

impl PageRange
{
    pub fn pagecount(&self) -> usize { self.pages.iter().filter(|page| **page).count() }

    pub fn new(
        range: &str,
        pagecount: usize,
    ) -> Option<PageRange>
    {
        if range == "" || range == "-" {
            return Some(PageRange{pages: vec![true; pagecount]);
        }
        let range = range.trim();

        let steps: Vec<&str> = range.split(',').collect();

        let mut page_singles: Vec<u32> = steps.iter().filter_map(|s| s.parse().ok()).collect();

        let page_differences: Vec<PageDifference> =
            steps.iter().filter_map(|s| PageDifference::from_str(s).ok()).collect();

        for diff in page_differences.iter() {
            for page in diff.minuend ..= diff.subtrahend {
                page_singles.push(page);
            }
        }
        debug!("page_singles: {:?}", page_singles);
        let mut pages: Vec<bool> = vec![false; pagecount];
        for page in page_singles.iter() {
            if *page <= pagecount as u32 {
                pages[*page as usize - 1] = true;
            }
        }
        if pages.iter().all(|page| !page) {
            None
        }
        else {
            Some(PageRange {
                pages,
            })
        }
    }
}

impl fmt::Display for PageRange
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        if self.pages.iter().all(|&x| x) {
            return write!(f, "");
        }

        let mut page = 1;
        while page < self.pages.len() + 1 {
            let mut diff = 0;
            while page + diff <= self.pages.len() {
                if self.pages[page - 1 + diff] {
                    if self.pages[page + diff] {
                        while page + diff < self.pages.len() && self.pages[page + diff] {
                            diff += 1;
                        }
                        write!(f, "{}-{},", page, page + diff)?;
                        page = page + diff + 2;
                        diff = 0;
                    }
                    else {
                        write!(f, "{},", page)?;
                        page += 2;
                    }
                }
                else {
                    page += 1;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests
{
    use jobs::options::pagerange::PageRange;
    pub fn print_range(
        range_str: &str,
        pagecount: usize,
    )
    {
        println!("################################");
        println!("range_str: {}, pagecount: {}", range_str, pagecount);
        match PageRange::new(range_str, pagecount) {
            Some(range) => {
                println!("range: {:?}, fmt: {}, count: {}", range, range, range.pagecount());
                let mut range_str = format!("{}", range);
                let _char = range_str.pop();
                println!("popped: {}", range_str);
            },
            None => println!("None"),
        }
    }
    #[test]
    pub fn no_err_from_str()
    {
        print_range("1,2-3,7-20,21-29", 32);

        print_range("2,3", 5);
        print_range("2-3", 4);
        print_range("2-4", 4);

        print_range("1,3-4,7-10", 11);

        print_range("1, 3 -2,7-10", 4);

        print_range("1df3-4,7-10", 17);

        print_range("1-2-4,7-10, 11-12", 13);

        print_range("1-7,10,11,12-16", 13);
        print_range("", 1);
        print_range("-", 7);
    }
}
