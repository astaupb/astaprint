use diesel::prelude::*;
use user::table::*;

pub fn check_card_credentials(card: u64, pin: u32, connection: &MysqlConnection) -> Option<u32>
{

}
