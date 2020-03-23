use mysql::journal::{
    Journal,
    JournalToken,
    PrintJournal,
};

use crate::job::options::JobOptions;

/// represenation of the journal information of a print job
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrintJournalResponse
{
    pub pages: u16,
    pub colored: u16,
    pub score: i16,
    pub device_id: u32,
    pub options: JobOptions,
}

impl<'a> From<&'a PrintJournal> for PrintJournalResponse
{
    fn from(print_journal: &PrintJournal) -> PrintJournalResponse
    {
        PrintJournalResponse{
            pages: print_journal.pages,
            colored: print_journal.colored,
            score: print_journal.score,
            device_id: print_journal.device_id,
            options: JobOptions::from(&print_journal.options[..]),
        }
    }
}

/// representation of a single transaction in the journal
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalResponse
{
    pub user_id: u32,
    pub value: i32,
    pub admin_id: Option<u32>,
    pub description: String,
    pub print: Option<PrintJournalResponse>,
    pub timestamp: String,
    pub created: i64,
}

impl<'a> From<&'a Journal> for JournalResponse
{
    fn from(journal: &Journal) -> JournalResponse
    {
        JournalResponse{
            user_id: journal.user_id,
            value: journal.value,
            admin_id: journal.admin_id,
            description: journal.description.clone(),
            print: None,
            timestamp: format!("{}", journal.created),
            created: journal.created.timestamp(),
        }
    }
}

impl<'a> From<&'a(Journal, Option<PrintJournal>)> for JournalResponse
{
    fn from(journal: &(Journal, Option<PrintJournal>)) -> JournalResponse
    {
        JournalResponse{
            user_id: journal.0.user_id,
            value: journal.0.value,
            admin_id: journal.0.admin_id,
            description: journal.0.description.clone(),
            print: journal.1.as_ref().map(PrintJournalResponse::from),
            timestamp: format!("{}", journal.0.created),
            created: journal.0.created.timestamp(),
        }
    }
}

/// representation of a journal token
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
