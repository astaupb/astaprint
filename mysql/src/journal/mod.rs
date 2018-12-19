pub mod select;

use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

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
    used_by: Option<u32>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}
