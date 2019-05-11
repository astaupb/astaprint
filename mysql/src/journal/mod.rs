pub mod select;
pub mod insert;
pub mod update;

use crate::schema::*;
use chrono::NaiveDateTime;
use std::fmt;

joinable!(journal -> user (user_id));
joinable!(journal -> print_journal (print_id));
joinable!(journal -> admin (admin_id));

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal"]
pub struct Journal
{
    pub id: u32,
    pub user_id: u32,
    pub credit: i32,
    pub value: i32,
    pub print_id: Option<u32>,
    pub admin_id: Option<u32>,
    pub description: String,
    pub created: NaiveDateTime,
}

joinable!(print_journal -> printers (device_id));

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "print_journal"]
pub struct PrintJournal
{
    pub id: u32,
    pub job_id: u32,
    pub pages: u16,
    pub colored: u16,
    pub score: i16,
    pub device_id: u32,
    pub options: Vec<u8>,
    pub created: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_tokens"]
pub struct JournalToken
{
    pub id: u32,
    pub value: u32,
    pub content: String,
    pub used: bool,
    pub used_by: Option<u32>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl fmt::Display for Journal
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}{}{}{}{}", self.id, self.user_id, self.value, self.description, self.created)
    }
}
