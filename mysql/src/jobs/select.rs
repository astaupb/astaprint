use crate::{
    jobs::*,
    schema::*,
};
use diesel::prelude::*;


pub fn select_jobs(connection: &MysqlConnection) -> QueryResult<Vec<Job>>
{
    jobs::table.select(jobs::all_columns).load(connection)
}

pub fn select_all_jobs_of_user(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Vec<(u32, Vec<u8>, Vec<u8>, NaiveDateTime)>>
{
    jobs::table
        .select((jobs::id, jobs::info, jobs::options, jobs::created))
        .filter(jobs::user_id.eq(user_id))
        .load(connection)
}

pub fn select_single_job_of_user(
    job_id: u32,
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<(u32, Vec<u8>, Vec<u8>, NaiveDateTime)>>
{
    jobs::table
        .select((jobs::id, jobs::info, jobs::options, jobs::created))
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(job_id))
        .first(connection)
        .optional()
}

pub fn select_job_info(
    job_id: u32,
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::info)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(job_id))
        .first(connection)
        .optional()
}

pub fn select_job_options(
    job_id: u32,
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::options)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(job_id))
        .first(connection)
        .optional()
}

pub fn select_pdf(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::pdf)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(id))
        .first(connection)
        .optional()
}
pub fn select_preview_0(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::preview_0)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(id))
        .first(connection)
        .optional()
}
pub fn select_preview_1(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::preview_1)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(id))
        .first(connection)
}
pub fn select_preview_2(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::preview_2)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(id))
        .first(connection)
}

pub fn select_preview_3(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<Option<Vec<u8>>>
{
    jobs::table
        .select(jobs::preview_3)
        .filter(jobs::user_id.eq(user_id))
        .filter(jobs::id.eq(id))
        .first(connection)
}
