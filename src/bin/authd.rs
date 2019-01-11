extern crate legacy;
use legacy::authd;

fn main()
{
    authd().expect("running authd");
}
