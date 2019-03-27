pub mod select;
pub mod insert;
pub mod update;

use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::fmt;

joinable!(journal -> user (user_id));

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal"]
pub struct Journal
{
    pub id: u32,
    pub user_id: u32,
    pub value: BigDecimal,
    pub description: String,
    pub created: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_digest"]
pub struct JournalDigest
{
    pub id: u32,
    pub digest: Vec<u8>,
    pub credit: BigDecimal,
    pub created: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_tokens"]
pub struct JournalToken
{
    pub id: u32,
    pub value: BigDecimal,
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
