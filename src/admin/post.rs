use rocket::http::Status;
use rocket_contrib::json::Json;
use admin::guard::AdminGuard;
use mysql::admin::select::select_admin_by_login;
use sodium::PasswordHash;
use chrono::NaiveDate;
use diesel::prelude::*;
use admin::Admin;

#[derive(Deserialize, Debug, Clone)]
pub struct NewAdmin
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
    pub owner: bool,
}

#[post("/", data = "<new>")]
pub fn post_new_admin(admin: AdminGuard, new: Json<NewAdmin>) -> QueryResult<Status>
{
    if select_admin_by_login(&new.login, &admin.connection).is_ok() {
        return Ok(Status::new(472, "login already taken"));
    }
    let new = new.into_inner();

    let (hash, salt) = PasswordHash::create(&new.password);

    let new_admin = Admin {
        first_name: new.first_name,
        last_name: new.last_name,
        login: Some(new.login),
        hash: Some(hash),
        salt: Some(salt),
        service: false,
        locked: false,
        owner: new.owner,
        expires: NaiveDate::from_yo(2019, 1),
    };

    new_admin.insert(&admin.connection)?;

    Ok(Status::new(204, "Success - No Content"))
}