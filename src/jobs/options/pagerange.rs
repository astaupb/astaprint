/// AStAPrint - pagerange.rs
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
use std::str::FromStr;

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
            return Err(());
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

impl<'a> FromStr for PageRange
{
    type Err = ();

    fn from_str(range: &str) -> Result<PageRange, ()>
    {
        let range = range.trim();

        let steps: Vec<&str> = range.split(',').collect();

        let mut page_singles: Vec<u32> = steps.iter().filter_map(|s| s.parse().ok()).collect();

        let page_differences: Vec<PageDifference> =
            steps.iter().filter_map(|s| PageDifference::from_str(s).ok()).collect();

        for diff in page_differences.iter() {
            for page in diff.minuend..=diff.subtrahend {
                page_singles.push(page);
            }
        }
        let pagecount = match page_singles.iter().max() {
            Some(int) => *int as usize,
            None => return Err(()),
        };

        let mut pages: Vec<bool> = vec![false; pagecount];
        for page in page_singles.iter() {
            pages[*page as usize - 1] = true;
        }
        Ok(PageRange {
            pages,
        })
    }
}


#[test]
pub fn pagerange_is_valid()
{
    let range = PageRange::from_str("1,2-3,7-20,21-29");
    println!("{:?}", range);
    assert!(range.is_ok());

    let range = PageRange::from_str("1,3-4,7-10");
    println!("{:?}", range);
    assert!(range.is_ok());

    let range = PageRange::from_str("1, 3 -2,7-10");
    println!("{:?}", range);
    assert!(range.is_ok());

    let range = PageRange::from_str("1df3-4,7-10");
    println!("{:?}", range);
    assert!(range.is_ok());

    let range = PageRange::from_str("1-2-4,7-10, 11-12");
    println!("{:?}", range);
    assert!(range.is_ok());
}
