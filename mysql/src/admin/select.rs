use crate::{
    admin::*,
    schema::*,
};
use diesel::prelude::*;

pub fn select_admin(connection: &MysqlConnection) -> QueryResult<Vec<Admin>>
{
    admin::table.select(admin::all_columns).load(connection)
}

pub fn select_admin_tokens(
    connection: &MysqlConnection,
) -> QueryResult<Vec<AdminToken>>
{
    admin_tokens::table.select(admin_tokens::all_columns).load(connection)
}
