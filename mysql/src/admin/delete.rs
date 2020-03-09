use crate::{
    schema::*,
};
use diesel::prelude::*;

pub fn delete_admin_by_id(id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        admin::table
            .filter(admin::id.eq(id))
        )
        .execute(connection)
}

pub fn delete_admin_token_by_id(id: u32, admin_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        admin_tokens::table
            .filter(admin_tokens::id.eq(id))
            .filter(admin_tokens::admin_id.eq(admin_id))
        )
        .execute(connection)
}

pub fn delete_admin_tokens_by_admin_id(admin_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        admin_tokens::table
            .filter(admin_tokens::admin_id.eq(admin_id))
        )
        .execute(connection)
}