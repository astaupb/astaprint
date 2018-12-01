use rocket::{
    http::Status,
    request::{
        self,
        FromRequest,
    },
    Outcome,
    Request,
    State,
};

use diesel::{
    self,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
        PooledConnection,
    },
};


impl<'a, 'r> FromRequest<'a, 'r> for JournalToken
{
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserGuard, ()>
    {
        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
        };

        Outcome::Success(

    }
}
