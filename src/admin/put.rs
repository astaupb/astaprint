use admin::guard::AdminGuard;
use mysql::user::update::update_locked;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[put("/users/<id>/locked", data = "<locked>")]
pub fn change_user_locked(id: u32, locked: Json<bool>, admin: AdminGuard)
    -> Status
{
    let locked = locked.into_inner();
    match update_locked(id, locked, &admin.connection) {
        Ok(1) => {
            info!("user {} locked: {}", id, locked);
            Status::new(205, "Success - Reset Content")
        },
        err => {
            error!("{:?}", err);
            Status::new(500, "Internal Server Error")
        },
    }
}
