#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction
{
    pub value: f64,
    pub description: String,
    pub timestamp: i64,
}