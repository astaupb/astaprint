use diesel::prelude::*;

use crate::{
    schema::*,
    user::*,
};

pub fn select_user(connection: &MysqlConnection) -> QueryResult<Vec<User>>
{
    user::table.select(user::all_columns).load(connection)
}

pub fn select_user_tokens(
    connection: &MysqlConnection,
) -> QueryResult<Vec<UserToken>>
{
    user_tokens::table.select(user_tokens::all_columns).load(connection)
}

pub fn select_user_tokens_by_user_id(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Vec<UserToken>>
{
    user_tokens::table
        .select(user_tokens::all_columns)
        .filter(user_tokens::user_id.eq(user_id))
        .load(connection)
}

pub fn select_user_id_by_name(
    name: &str,
    connection: &MysqlConnection,
) -> QueryResult<Option<u32>>
{
    user::table
        .select(user::id)
        .filter(user::name.eq(name))
        .first(connection)
        .optional()
}

pub fn select_user_name_by_id(
    id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<String>>
{
    user::table
        .select(user::name)
        .filter(user::id.eq(id))
        .first(connection)
        .optional()
}

pub fn select_user_id_by_card_credentials(
    card: Vec<u8>,
    pin: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<u32>>
{
    user::table
        .select(user::id)
        .filter(user::card.eq(Some(card)))
        .filter(user::pin.eq(Some(pin)))
        .first(connection)
        .optional()
}

pub fn select_user_id_by_hash(hash: Vec<u8>, connection: &MysqlConnection) -> QueryResult<Option<u32>>
{
    user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::hash.eq(hash))
        .first(connection)
        .optional()
}

pub fn select_user_hash_by_id(user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    user::table
        .select(user::hash)
        .filter(user::id.eq(user_id))
        .first(connection)
        .optional()
}

pub fn select_user_token_id_by_hash(
    hash: Vec<u8>,
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<u32>>
{
    user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::user_id.eq(user_id))
        .filter(user_tokens::hash.eq(hash))
        .first(connection)
        .optional()
}

pub fn select_user_by_name(
    name: &str,
    connection: &MysqlConnection,
) -> QueryResult<Option<User>>
{
    user::table
        .select(user::all_columns)
        .filter(user::name.eq(name))
        .first(connection)
        .optional()
}

pub fn select_hash_and_salt(user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<(Vec<u8>, Vec<u8>)>>
{
    user::table
        .select((user::hash, user::salt))
        .filter(user::id.eq(user_id))
        .first(connection)
        .optional()
}