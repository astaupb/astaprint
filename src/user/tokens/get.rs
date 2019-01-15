use diesel::QueryResult;
use rocket_contrib::json::Json;

use user::{
    guard::UserGuard,
    response::UserTokenResponse,
};

use mysql::user::{
    select::*,
    UserToken,
};


#[get("/")]
pub fn get_all_tokens(user: UserGuard)
    -> QueryResult<Json<Vec<UserTokenResponse>>>
{
    let tokens: Vec<UserToken> =
        select_user_tokens_by_user_id(user.id, &user.connection)?;

    info!("{} fetched all tokens", user.id);

    Ok(Json(tokens.iter().map(|row| UserTokenResponse::from(row)).collect()))
}

#[get("/<token_id>")]
pub fn get_single_token(
    user: UserGuard,
    token_id: u32,
) -> QueryResult<Option<Json<UserTokenResponse>>>
{
    let token: Option<UserToken> =
        select_single_user_token_optional(user.id, token_id, &user.connection)?;
    Ok(token.map(|x| Json(UserTokenResponse::from(&x))))
}
