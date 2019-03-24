#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CounterValues
{
    total: i64,
    copy_total: i64,
    copy_bw: i64,
    print_total: i64,
    print_bw: i64,
}
