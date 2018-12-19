pub mod select;
use crate::schema::*;
use chrono::{
    NaiveDate,
    NaiveDateTime,
};
#[derive(Identifiable, Queryable, Insertable, Debug)]
#[table_name = "admin"]
pub struct Admin
{
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub is_service: bool,
    pub expires: NaiveDate,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "admin_tokens"]
pub struct AdminToken
{
    pub id: u32,
    pub user_id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub created: NaiveDateTime,
}
