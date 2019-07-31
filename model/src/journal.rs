use mysql::journal::{
    Journal,
    JournalToken,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalResponse
{
    pub value: i32,
    pub admin_id: Option<u32>,
    pub without_receipt: bool,
    pub description: String,
    pub timestamp: String,
    pub created: i64,
}

impl<'a> From<&'a Journal> for JournalResponse
{
    fn from(journal: &Journal) -> JournalResponse
    {
        JournalResponse{
            value: journal.value,
            admin_id: journal.admin_id,
            without_receipt: true,
            description: journal.description.clone(),
            timestamp: format!("{}", journal.created),
            created: journal.created.timestamp(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct JournalTokenResponse
{
    id: u32,
    value: u32,
    content: String,
    used: bool,
    used_by: Option<u32>,
    created: i64,
    updated: i64,
}

impl<'a> From<&'a JournalToken> for JournalTokenResponse
{
    fn from(token: &JournalToken) -> JournalTokenResponse
    {
        JournalTokenResponse {
            id: token.id,
            value: token.value,
            content: token.content.clone(),
            used: token.used,
            used_by: token.used_by,
            created: token.created.timestamp(),
            updated: token.updated.timestamp(),
        }
    }
}
