#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction
{
    pub value: i32,
    pub description: String,
    pub without_receipt: bool,
    pub user_id: u32,
    pub admin_id: Option<u32>,
    pub timestamp: String,
}