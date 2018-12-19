pub mod select;
pub mod update;
pub mod insert;
pub mod delete;

use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
pub struct Job
{
    pub id: u32,
    pub user_id: u32,
    pub info: Vec<u8>,
    pub options: Vec<u8>,
    pub data: Vec<u8>,
    pub preview_0: Vec<u8>,
    pub preview_1: Option<Vec<u8>>,
    pub preview_2: Option<Vec<u8>>,
    pub preview_3: Option<Vec<u8>>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
