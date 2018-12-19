use diesel::{
    prelude::*,
    insert_into,
};
use crate::schema::*;

pub fn insert_into_jobs(
    user_id: u32,
    info: Vec<u8>,
    options: Vec<u8>,
    data: Vec<u8>,
    preview_0: Vec<u8>,
    preview_1: Option<Vec<u8>>,
    preview_2: Option<Vec<u8>>,
    preview_3: Option<Vec<u8>>,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(jobs::table)
        .values((
            jobs::user_id.eq(user_id),
            jobs::info.eq(info),
            jobs::options.eq(options),
            jobs::data.eq(data),
            jobs::preview_0.eq(preview_0),
            jobs::preview_1.eq(preview_1),
            jobs::preview_2.eq(preview_2),
            jobs::preview_3.eq(preview_3),
        ))
        .execute(connection)
}