use std::env;

pub struct Environment
{
    pub userdir: String,
    pub spooldir: String,
}

impl Default for Environment
{
    fn default() -> Environment
    {
        Environment {
            spooldir: env::var("ASTAPRINT_SPOOL_DIR")
                .expect("reading ASTAPRINT_SPOOL_DIR from environment"),
            userdir: env::var("ASTAPRINT_USER_DIR").expect("reading ASTAPRINT_USER_DIR from environment"),
        }
    }
}
