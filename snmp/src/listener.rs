use std::{
    thread,
    sync::mpsc,
};

use crate::{
    tool::snmptool,
    CounterValues,
};

pub fn listen(device_id: u32) -> mpsc::Receiver<CounterValues>
{
    let (sender, receiver) = mpsc::channel();
    {
        let sender = sender.clone();
        thread::spawn(move || {
            let listener = snmptool(&[&format!("{}", device_id), "listen"])
                .expect("spawning listener");
            let stdout = listener.stdout
                .expect("getting stdout handler from listener");
            for line in stdout.lines() {
                println!("{}", stdout) 
            }
        });
        
    }
    receiver
}
