use admin::guard::AdminGuard;
use diesel::prelude::*;
use model::job::options::JobOptions;
use mysql::user::{
    select::select_user,
    User,
};
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
#[get("/users")]
pub fn get_all_users(admin: AdminGuard) -> QueryResult<Json<Vec<UserResponse>>>
{
    Ok(Json(
        select_user(&admin.connection)?
            .iter()
            .map(|row| UserResponse::from(row))
            .collect(),
    ))
}
