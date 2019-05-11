#[macro_use]
extern crate serde_derive;

extern crate log;

extern crate bincode;

pub mod job;
pub mod task;
pub mod journal;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    use super::task::dispatcher::DispatcherTask;
    #[test]
    fn deserialize_dispatcher_task()
    {
        let mut task = DispatcherTask{
            user_id: 18616,
            filename: String::from("abc"),
            keep: true,
            uid: vec![7; 20],
        }; 
        let ser = bincode::serialize(&task).unwrap();
        println!("{:x?}", ser);
        task.keep = false;
        let ser = bincode::serialize(&task).unwrap();
        println!("{:x?}", ser);
    }
}
