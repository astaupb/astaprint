use admin::guard::AdminGuard;
use diesel::prelude::*;
use model::{
    job::options::JobOptions,
    journal::Transaction,
};
use mysql::user::{
    select::{select_user_with_limit_offset, select_user_by_id},
    User,
};
use legacy::tds::{
    get_credit, get_journal_of_user,
};

use journal::credit::decimal_to_cent;

use rocket_contrib::json::Json;

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse
{
    pub id: u32,
    pub name: String,
    pub options: Option<JobOptions>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub created: String,
    pub updated: String,
}

impl<'a> From<&'a User> for UserResponse
{
    fn from(user: &User) -> UserResponse
    {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            options: user.options.clone().map(|x| {
                bincode::deserialize(&x[..]).expect("deserializing JobOption")
            }),
            card: user.card,
            pin: user.pin,
            locked: user.locked,
            created: format!("{}", user.created),
            updated: format!("{}", user.updated),
        }
    }
}

#[get("/users?<limit>&<offset>")]
pub fn get_all_users(limit: Option<i64>, offset: Option<i64>, admin: AdminGuard) -> QueryResult<Json<Vec<UserResponse>>>
{
    Ok(Json(
        select_user_with_limit_offset(
            limit.unwrap_or(50),
            offset.unwrap_or(0),
            &admin.connection,
        )?
        .iter()
        .map(|row| UserResponse::from(row))
        .collect(),
    ))
}

#[get("/users/<id>")]
pub fn get_user_as_admin(id: u32, admin: AdminGuard) -> QueryResult<Json<UserResponse>>
{
    Ok(Json(UserResponse::from(
        &select_user_by_id(id, &admin.connection)?
    )))
}

#[get("/users/<id>/journal?<desc>&<offset>&<limit>")]
pub fn get_user_journal_as_admin(id: u32, desc: Option<bool>, offset: Option<i32>, limit: Option<u32>, _admin: AdminGuard) -> Json<Vec<Transaction>>
{
    Json(get_journal_of_user(
        id,
        desc.unwrap_or(true),
        offset.unwrap_or(0),
        limit.unwrap_or(u32::max_value()),
    ))
}

#[get("/users/<id>/credit")]
pub fn get_user_credit_as_admin(id: u32) -> Json<i32>
{
    Json(decimal_to_cent(get_credit(id)))
}