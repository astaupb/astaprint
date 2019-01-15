/// AStAPrint-Common - Pageinfo.rs
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

#[derive(Debug, PartialEq)]
pub enum PageSize
{
    A4,
    A3,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageOrientation
{
    Portrait,
    Landscape,
}

#[derive(Debug, PartialEq)]
pub enum Is<T>
{
    Valid(T),
    Almost(T),
}

#[derive(Debug, PartialEq)]
pub struct PageInfo
{
    pub size: Is<PageSize>,
    pub orientation: PageOrientation,
}

impl PageInfo
{
    pub fn from_points(w: f64, h: f64) -> PageInfo
    {
        let orientation = if w > h {
            PageOrientation::Landscape } else {
            PageOrientation::Portrait
        };

        let (x, y) = match orientation {
            PageOrientation::Portrait => (h, w),
            PageOrientation::Landscape => (w, h),
        };

        assert!(x >= y);

        let mut size = Is::Almost(PageSize::A4);

        if x >= 841.0 && x <= 843.0 && y >= 594.0 && y <= 596.0 {
            size = Is::Valid(PageSize::A4);
        } else if x >= 1189.0 && x <= 1191.0 && y >= 841.0 && y <= 843.0 {
            size = Is::Valid(PageSize::A3);
        } else if x > 1016.0 {
            size = Is::Almost(PageSize::A3);
        }

        PageInfo {
            size,
            orientation,
        }
    }

    pub fn from_multiple(pageinfo: Vec<PageInfo>) -> PageInfo
    {
        if pageinfo.iter().all(|info| info.size == Is::Valid(PageSize::A4)) {
            return PageInfo {
                size: Is::Valid(PageSize::A4),
                orientation: pageinfo[0].orientation.clone(),
            };
        }

        if pageinfo.iter().all(|info| info.size == Is::Valid(PageSize::A3)) {
            return PageInfo {
                size: Is::Valid(PageSize::A3),
                orientation: pageinfo[0].orientation.clone(),
            };
        }

        let mut a4_count = 0;

        pageinfo
            .iter()
            .filter(|info| {
                info.size == Is::Valid(PageSize::A4)
                    || info.size == Is::Almost(PageSize::A4)
            })
            .for_each(|_info| a4_count += 1);

        let mut portrait_count = 0;

        pageinfo
            .iter()
            .filter(|info| info.orientation == PageOrientation::Portrait)
            .for_each(|_info| portrait_count += 1);

        PageInfo {
            size: Is::Almost(if pageinfo.len() - a4_count > a4_count {
                PageSize::A3 } else {
                PageSize::A4
            }),
            orientation: if pageinfo.len() - portrait_count > portrait_count
                {
                    PageOrientation::Landscape
                } else {
                    PageOrientation::Portrait
            },
        }
    }
}
