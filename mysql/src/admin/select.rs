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

pub fn select_admin_token_id_by_hash(admin_id: u32, hash: Vec<u8>, connection: &MysqlConnection)
    -> QueryResult<u32>
{
    admin_tokens::table
        .select(admin_tokens::id)
        .filter(admin_tokens::admin_id.eq(admin_id))
        .filter(admin_tokens::hash.eq(hash))
        .first(connection)
}

pub fn select_admin_hash_by_id(admin_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    admin::table
        .select(admin::hash)
        .filter(admin::id.eq(admin_id))
        .first(connection)
}
pub fn select_admin_by_login(login: &str, connection: &MysqlConnection)
    -> QueryResult<Admin>
{
    admin::table
        .select(admin::all_columns)
        .filter(admin::login.eq(Some(login)))
        .first(connection)
}

pub fn select_admin_token_ip_and_location_by_id(token_id: u32, connection: &MysqlConnection) -> QueryResult<(String, String)> {
    admin_tokens::table
        .select((admin_tokens::ip, admin_tokens::location))
        .filter(admin_tokens::id.eq(token_id))
        .first(connection)
}
