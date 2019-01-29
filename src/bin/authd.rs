extern crate legacy;
use legacy::authd;
extern crate logger;
use logger::Logger;
fn main()
{
    Logger::init().expect("initializing Logger");
    authd().expect("running authd");
}
