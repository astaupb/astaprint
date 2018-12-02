use diesel::{
    self,
    prelude::*,
};
use rocket_contrib::Json;

use user::{
    guard::UserGuard,
    response::UserTokenResponse,
    tokens::{
        table::*,
        UserToken,
    },
};


#[get("/")]
pub fn get_all_tokens(user: UserGuard) -> Result<Json<Vec<UserTokenResponse>>, diesel::result::Error>
{
    let tokens: Vec<UserToken> = user_tokens::table
        .select(user_tokens::all_columns)
        .filter(user_tokens::user_id.eq(user.id))
        .load(&user.connection)?;

    info!("{} fetched all tokens", user.id);

    Ok(Json(tokens.iter().map(|row| UserTokenResponse::from(row)).collect()))
}

#[get("/<token_id>")]
pub fn get_single_token(
    user: UserGuard,
    token_id: u32,
) -> Result<Option<Json<UserTokenResponse>>, diesel::result::Error>
{
    let token: Option<UserToken> = user_tokens::table
        .select(user_tokens::all_columns)
        .filter(user_tokens::id.eq(token_id))
        .first(&user.connection)
        .optional()?;

    Ok(token.map(|x| Json(UserTokenResponse::from(&x))))
}
