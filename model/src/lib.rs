#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod job;
pub mod task;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
