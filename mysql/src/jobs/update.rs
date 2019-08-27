use diesel::{
    prelude::*,
    update,
};

use crate::schema::*;

pub fn update_job_options(
    job_id: u32,
    user_id: u32,
    value: Vec<u8>,
    connection: &MysqlConnection
) -> QueryResult<usize>
{
    update(
        jobs::table
            .filter(jobs::user_id.eq(user_id))
            .filter(jobs::id.eq(job_id))
        )
        .set(jobs::options.eq(value))
        .execute(connection)
}

pub fn update_job_options_by_id(
    id: u32,
    value: Vec<u8>,
    connection: &MysqlConnection
) -> QueryResult<usize>
{
    update(
        jobs::table
            .filter(jobs::id.eq(id))
        )
        .set(jobs::options.eq(value))
        .execute(connection)
}

pub fn update_job_info(
    id: u32,
    user_id: u32,
    value: Vec<u8>,
    connection: &MysqlConnection
) -> QueryResult<usize>
{
    update(
        jobs::table
            .filter(jobs::user_id.eq(user_id))
            .filter(jobs::id.eq(id))
        )
        .set(jobs::info.eq(value))
        .execute(connection)
}

pub fn update_job_info_by_id(
    id: u32,
    value: Vec<u8>,
    connection: &MysqlConnection
) -> QueryResult<usize>
{
    update(
        jobs::table
            .filter(jobs::id.eq(id))
        )
        .set(jobs::info.eq(value))
        .execute(connection)
}
