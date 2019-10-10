#[macro_use]
extern crate serde_derive;

#[macro_use]
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
            keep: Some(true),
            color: Some(true),
            a3: Some(true),
            duplex: Some(0),
            uid: vec![7; 20],
        }; 
        let ser = bincode::serialize(&task).unwrap();
        println!("{:x?}", ser);
        task.keep = Some(false);
        let ser = bincode::serialize(&task).unwrap();
        println!("{:x?}", ser);
        task.keep = None;
        let ser = bincode::serialize(&task).unwrap();
        println!("{:x?}", ser);
    }
}
