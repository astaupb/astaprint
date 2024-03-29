use diesel::prelude::*;

use crate::{
    schema::*,
    user::*,
};

pub fn select_user(connection: &MysqlConnection) -> QueryResult<Vec<User>>
{
    user::table.select(user::all_columns).load(connection)
}

pub fn select_user_id(connection: &MysqlConnection) -> QueryResult<Vec<u32>>
{
        user::table.select(user::id).order(user::id.desc()).load(connection)
}

pub fn select_user_with_limit_offset(limit: i64, offset: i64, connection: &MysqlConnection) -> QueryResult<Vec<User>>
{
    user::table
        .select(user::all_columns)
        .order(user::id.desc())
        .limit(limit)
        .offset(offset)
        .load(connection)
}

pub fn select_user_by_id(user_id: u32, connection: &MysqlConnection) -> QueryResult<User>
{
    user::table.select(user::all_columns).filter(user::id.eq(user_id)).first(connection)
}

pub fn select_user_credit_by_id(id: u32, connection : &MysqlConnection) -> QueryResult<i32>
{
    user::table.select(user::credit).filter(user::id.eq(id)).first(connection)
}

pub fn select_user_pin_by_id(user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<u32>>
{
    user::table.select(user::pin).filter(user::id.eq(user_id)).first(connection)
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

pub fn select_single_user_token_optional(
    user_id: u32,
    token_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<UserToken>>
{
    user_tokens::table
        .select(user_tokens::all_columns)
        .filter(user_tokens::id.eq(token_id))
        .filter(user_tokens::user_id.eq(user_id))
        .first(connection)
        .optional()
}

pub fn select_token_ids_of_user(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Vec<u32>>
{
    user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::user_id.eq(user_id))
        .load(connection)
}


pub fn select_user_id_by_name(
    name: &str,
    connection: &MysqlConnection,
) -> QueryResult<u32>
{
    user::table
        .select(user::id)
        .filter(user::name.eq(name))
        .first(connection)
}

pub fn select_user_id_by_name_optional(
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

pub fn select_user_id_by_email_optional(
    email: &str,
    connection: &MysqlConnection,
    ) -> QueryResult<Option<u32>>
{
    user::table
        .select(user::id)
        .filter(user::email.eq(email))
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

pub fn select_user_hash_by_id(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Vec<u8>>
{
    user::table
        .select(user::hash)
        .filter(user::id.eq(user_id))
        .first(connection)
}

pub fn select_hash_and_salt(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<(Vec<u8>, Vec<u8>)>
{
    user::table
        .select((user::hash, user::salt))
        .filter(user::id.eq(user_id))
        .first(connection)
}

pub fn select_user_token_id_by_hash(
    user_id: u32,
    hash: Vec<u8>,
    connection: &MysqlConnection,
) -> QueryResult<u32>
{
    user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::user_id.eq(user_id))
        .filter(user_tokens::hash.eq(hash))
        .first(connection)
}

pub fn select_user_id_by_card_credentials(
    card: u64,
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

pub fn select_user_by_name(name: &str, connection: &MysqlConnection) -> QueryResult<User>
{
    user::table
        .select(user::all_columns)
        .filter(user::name.eq(name))
        .first(connection)
}

pub fn select_user_id_by_hash_optional(hash: Vec<u8>, connection: &MysqlConnection) -> QueryResult<Option<u32>>
{
    user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::hash.eq(hash))
        .first(connection)
        .optional()
}

pub fn select_user_by_name_optional(
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

pub fn select_user_email_by_id(user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<String>>
{
    user::table
        .select(user::email)
        .filter(user::id.eq(user_id))
        .first(connection)
}

pub fn select_user_info_by_id(user_id: u32, connection: &MysqlConnection) -> QueryResult<UserSelect>
{
    user::table
        .select((user::name, user::credit, user::card, user::pin, user::email, user::tou_accept))
        .filter(user::id.eq(user_id))
        .first(connection)
}

pub fn select_user_options(user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    user::table
        .select(user::options)
        .filter(user::id.eq(user_id))
        .first(connection)
}

pub fn select_user_token_ip_and_location_by_id(token_id: u32, connection: &MysqlConnection) -> QueryResult<(String, String)> {
    user_tokens::table
        .select((user_tokens::ip, user_tokens::location))
        .filter(user_tokens::id.eq(token_id))
        .first(connection)
}
