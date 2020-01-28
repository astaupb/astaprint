use diesel::{
    prelude::*,
    insert_into,
};
use crate::schema::*;

#[derive(Debug, Clone)]
pub struct UserInsert<'a> {
    pub name: &'a str,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub email: Option<String>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
}

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

pub fn insert_into_user(
    user: UserInsert,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(user::table)
        .values((
            user::name.eq(user.name),
            user::locked.eq(user.locked),
            user::hash.eq(user.hash),
            user::salt.eq(user.salt),
            user::email.eq(user.email),
            user::card.eq(user.card),
            user::pin.eq(user.pin),
        ))
    .execute(connection)
}
