use diesel::{
    self,
    prelude::*,
    QueryResult,
};
use rocket::response::status::Reset;

use user::{
    guard::UserGuard,
    tokens::table::*,
};


#[delete("/")]
pub fn delete_all_tokens(user: UserGuard) -> QueryResult<Reset>
{
    diesel::delete(user_tokens::table.filter(user_tokens::user_id.eq(user.id)))
        .execute(&user.connection)?;

    info!("{} deleted all tokens", user.id);

    Ok(Reset)
}

#[delete("/<token_id>")]
pub fn delete_single_token(user: UserGuard, token_id: u32) -> QueryResult<Option<Reset>>
{
    let affected_rows = diesel::delete(
        user_tokens::table.filter(user_tokens::user_id.eq(user.id)).filter(user_tokens::id.eq(token_id)),
    )
    .execute(&user.connection)?;
    if affected_rows > 0 {
        info!("{} deleted token {}", user.id, &token_id);
        Ok(Some(Reset))
    } else {
        Ok(None)
    }
}
