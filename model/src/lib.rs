#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate bincode;

pub mod admin;
pub mod job;
pub mod task;
pub mod journal;
pub mod printer;
pub mod user;
pub mod ppd;


#[cfg(test)]
mod tests {
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
