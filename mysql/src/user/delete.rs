use diesel::{
    prelude::*,
    dsl::not,
};
use crate::schema::*; 

pub fn delete_all_tokens_of_user(user_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        user_tokens::table
            .filter(user_tokens::user_id.eq(user_id))
        )
        .execute(connection)
}

pub fn delete_all_tokens_of_user_except_one(user_id: u32, token_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        user_tokens::table
            .filter(user_tokens::user_id.eq(user_id))
            .filter(not(user_tokens::id.eq(token_id)))
        )
        .execute(connection)
}


pub fn delete_user_token_by_id(user_id: u32, token_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        user_tokens::table
            .filter(user_tokens::user_id.eq(user_id))
            .filter(user_tokens::id.eq(token_id))
        ).execute(connection)
}
