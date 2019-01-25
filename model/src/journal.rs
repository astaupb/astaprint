#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction
{
    pub value: i32,
    pub description: String,
    pub timestamp: i64,
}