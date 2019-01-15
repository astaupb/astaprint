use diesel::QueryResult;
use rocket::http::Status;
use user::guard::UserGuard;

use mysql::user::delete::*;

#[delete("/")]
pub fn delete_all_tokens(user: UserGuard) -> QueryResult<Status>
{
    let deleted = delete_all_tokens_of_user(user.id, &user.connection)?;
    info!("{} deleted {} tokens", user.id, deleted);

    Ok(Status::new(205, "Reset Content"))
}

#[delete("/<token_id>")]
pub fn delete_single_token(
    user: UserGuard,
    token_id: u32,
) -> QueryResult<Option<Status>>
{
    let affected_rows =
        delete_user_token_by_id(user.id, token_id, &user.connection)?;
    if affected_rows > 0 {
        info!("{} deleted token {}", user.id, &token_id);
        Ok(Some(Status::new(205, "Reset Content")))
    } else {
        Ok(None)
    }
}
