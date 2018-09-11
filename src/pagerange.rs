/// AStAPrint-Common - Pagerange.rs
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

pub fn page_range_is_valid(range: &str) -> bool {
    let range = range.replace(" ", "");
    let mut order = Vec::<u16>::new();
    let steps: Vec<&str> = range.split(",").collect();
    for ranges in steps {
        let pages: Vec<&str> = ranges.split("-").collect();
        if pages.len() > 2 {
            return false;
        }
        for page in pages {
            order.push(match page.parse() {
                Ok(int) => int,
                Err(_) => return false,
            });
        }
    }
    for i in 0..order.len() - 1 {
        if order[i] > order[i + 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod pdfinfo_tests {
    use super::*;
    #[test]
    fn check_page_ranges() {
        assert!(page_range_is_valid("1,2-3,7-20,21-29"));
        assert!(page_range_is_valid("1,3-4,7-10"));
        assert!(!page_range_is_valid("1, 3 -2,7-10"));
        assert!(!page_range_is_valid("1df3-4,7-10"));
        assert!(!page_range_is_valid("1-2-4,7-10, 11-12"));
    }
}
