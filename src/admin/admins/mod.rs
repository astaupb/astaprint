pub mod get;
pub mod post;
pub mod put;
pub mod delete;

use chrono::NaiveDate;
use diesel::prelude::*;
use mysql::admin::insert::insert_admin;

#[derive(Deserialize, Debug, Clone)]
pub struct NewAdmin
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Admin
{
    pub first_name: String,
    pub last_name: String,
    pub login: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub service: bool,
    pub locked: bool,
    pub expires: NaiveDate,
    pub created_by: Option<u32>,
}

impl Admin
{
    pub fn insert(self, connection: &MysqlConnection) -> QueryResult<usize>
    {
        insert_admin(
            (
                self.first_name,
                self.last_name,
                self.login,
                self.hash,
                self.salt,
                self.service,
                self.locked,
                self.expires,
                self.created_by,
            ),
            connection,
        )
    }
}
