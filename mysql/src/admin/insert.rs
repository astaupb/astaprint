use chrono::NaiveDate;
use diesel::{
    prelude::*,
    insert_into,
};

use crate::schema::*;

type AdminInsert = (String, String, Option<String>, Option<Vec<u8>>, Option<Vec<u8>>, bool, bool, NaiveDate, Option<u32>);

pub fn insert_admin(admin: AdminInsert, connection: &MysqlConnection)
    -> QueryResult<usize>
{
    insert_into(admin::table)
        .values((
            admin::first_name.eq(admin.0),
            admin::last_name.eq(admin.1),
            admin::login.eq(admin.2),
            admin::hash.eq(admin.3),
            admin::salt.eq(admin.4),
            admin::service.eq(admin.5),
            admin::locked.eq(admin.6),
            admin::expires.eq(admin.7),
            admin::created_by.eq(admin.8),
        ))
        .execute(connection)
}
pub fn insert_admin_token(token: (u32, String, String, String, Vec<u8>), connection: &MysqlConnection)
    -> QueryResult<usize>
{
    insert_into(admin_tokens::table)
        .values((
            admin_tokens::admin_id.eq(token.0),
            admin_tokens::user_agent.eq(token.1),
            admin_tokens::ip.eq(token.2),
            admin_tokens::location.eq(token.3),
            admin_tokens::hash.eq(token.4),
        ))
        .execute(connection)
}

