use crate::job::options::JobOptions;
use mysql::user::{
    User, UserToken,
};

/// representation of an user displayed to an admin
#[derive(Serialize, Debug, Clone)]
pub struct UserResponse
{
    pub id: u32,
    pub name: String,
    pub credit: i32,
    pub options: Option<JobOptions>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub email: Option<String>,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a User> for UserResponse
{
    fn from(user: &User) -> UserResponse
    {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            credit: user.credit,
            options: user
                .options
                .clone()
                .map(|x| JobOptions::from(&x[..])),
            card: user.card,
            pin: user.pin,
            locked: user.locked,
            email: user.email.clone(),
            created: user.created.timestamp(),
            updated: user.updated.timestamp(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Card
{
    pub sn: Option<u64>,
    pub pin: Option<u32>,
}

/// representation of a user token
#[derive(Serialize, Debug)]
pub struct UserTokenResponse
{
    pub id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a UserToken> for UserTokenResponse
{
    fn from(row: &UserToken) -> UserTokenResponse
    {
        UserTokenResponse {
            id: row.id,
            user_agent: row.user_agent.clone(),
            ip: row.ip.clone(),
            location: row.location.clone(),
            created: row.created.timestamp(),
            updated: row.updated.timestamp(),
        }
    }
}