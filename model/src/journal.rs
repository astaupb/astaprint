use mysql::journal::{
    Journal,
    JournalToken,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalResponse
{
    pub user_id: u32,
    pub credit: i32,
    pub value: i32,
    pub print_id: Option<u32>,
    pub admin_id: Option<u32>,
    pub description: String,
    pub timestamp: i64,
}

impl<'a> From<&'a Journal> for JournalResponse
{
    fn from(journal: &Journal) -> JournalResponse
    {
        JournalResponse{
            user_id: journal.user_id,
            credit: journal.credit,
            value: journal.value,
            print_id: journal.print_id,
            admin_id: journal.admin_id,
            description: journal.description.clone(),
            timestamp: journal.created.timestamp()
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
    created: String,
    updated: String,
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
            created: format!("{}", token.created),
            updated: format!("{}", token.updated),
        }
    }
}
