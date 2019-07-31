pub mod select;
pub mod delete;
pub mod insert;
pub mod update;

use crate::schema::*;
use chrono::NaiveDateTime;

joinable!(user_tokens -> user (user_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "user"]
pub struct User
{
    pub id: u32,
    pub name: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub credit: i32,
    pub options: Option<Vec<u8>>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "user_tokens"]
pub struct UserToken
{
    pub id: u32,
    pub user_id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub hash: Vec<u8>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
