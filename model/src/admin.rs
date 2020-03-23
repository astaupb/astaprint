use chrono::{
    NaiveTime,
};
use mysql::admin::{
    Admin, AdminToken,
};

/// represenation of a admin displayed to other admins
#[derive(Serialize, Debug, Clone)]
pub struct AdminResponse
{
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub service: bool,
    pub locked: bool,
    pub expires: i64,
    pub created_by: Option<u32>,
    pub created: i64,
    pub updated: i64,
}

impl From<&Admin> for AdminResponse
{
    fn from(admin: &Admin) -> AdminResponse
    {
        AdminResponse {
            id: admin.id,
            first_name: admin.first_name.clone(),
            last_name: admin.last_name.clone(),
            login: admin.login.clone(),
            service: admin.service,
            locked: admin.locked,
            expires: admin.expires.and_time(NaiveTime::from_hms(0, 0, 0)).timestamp(),
            created_by: admin.created_by,
            created: admin.created.timestamp(),
            updated: admin.updated.timestamp(),
        }
    }
}

/// representation of an admin token displayed to the admin itself
#[derive(Serialize, Debug)]
pub struct AdminTokenResponse
{
    pub id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a AdminToken> for AdminTokenResponse
{
    fn from(row: &AdminToken) -> AdminTokenResponse
    {
        AdminTokenResponse {
            id: row.id,
            user_agent: row.user_agent.clone(),
            ip: row.ip.clone(),
            location: row.location.clone(),
            created: row.created.timestamp(),
            updated: row.updated.timestamp(),
        }
    }
}