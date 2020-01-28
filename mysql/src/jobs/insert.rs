use diesel::{
    prelude::*,
    insert_into,
};
use crate::schema::*;

pub struct JobInsert {
    pub user_id: u32,
    pub info: Vec<u8>,
    pub options: Vec<u8>,
    pub pdf: Vec<u8>,
    pub preview_0: Vec<u8>,
    pub preview_1: Option<Vec<u8>>,
    pub preview_2: Option<Vec<u8>>,
    pub preview_3: Option<Vec<u8>>,
}

pub fn insert_into_jobs(
    job: JobInsert,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(jobs::table)
        .values((
            jobs::user_id.eq(job.user_id),
            jobs::info.eq(job.info),
            jobs::options.eq(job.options),
            jobs::pdf.eq(job.pdf),
            jobs::preview_0.eq(job.preview_0),
            jobs::preview_1.eq(job.preview_1),
            jobs::preview_2.eq(job.preview_2),
            jobs::preview_3.eq(job.preview_3),
        ))
        .execute(connection)
}
