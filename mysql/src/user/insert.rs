use diesel::{
    prelude::*,
    insert_into,
};
use crate::schema::*;

pub fn insert_into_user_tokens(
    user_id: u32,
    user_agent: &str,
    ip: &str,
    location: &str,
    hash: Vec<u8>,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(user_tokens::table)
        .values((
            user_tokens::user_id.eq(user_id),
            user_tokens::user_agent.eq(user_agent),
            user_tokens::ip.eq(ip),
            user_tokens::location.eq(location),
            user_tokens::hash.eq(hash),
        ))
        .execute(connection)
}