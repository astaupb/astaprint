use std::{
    io,
    process::{
        Child,
        Command,
        Stdio,
    },
};

use crate::{
    CounterValues,
    StatusValues,
};

#[derive(Debug)]
pub enum SnmpToolError
{
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

impl From<io::Error> for SnmpToolError
{
    fn from(err: io::Error) -> Self
    {
        SnmpToolError::IoError(err) 
    }
}

impl From<serde_json::Error> for SnmpToolError
{
    fn from(err: serde_json::Error) -> Self
    {
        SnmpToolError::SerdeError(err) 
    }
}


fn snmptool(args: &[&str]) -> io::Result<Child>
{
    Command::new("./snmptool").args(args).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
}

pub fn wake(device_id: u32) { let _wake = snmptool(&[&format!("{}", device_id), "wake"]); }

pub fn sleep(device_id: u32) { let _sleep = snmptool(&[&format!("{}", device_id), "sleep"]); }

pub fn clear(device_id: u32) -> io::Result<()> { let _clear = snmptool(&[&format!("{}", device_id), "clear"])?; Ok(()) }

pub fn status(device_id: u32) -> Result<StatusValues, SnmpToolError>
{
    let output = snmptool(&[&format!("{}", device_id), "status"])?.wait_with_output()?;
    let json = String::from_utf8_lossy(&output.stdout[..]);
    Ok(serde_json::from_str(&json)?)
}

pub fn counter(ip: &str) -> Result<CounterValues, SnmpToolError>
{
    let output = snmptool(&[ip, "counter"])?.wait_with_output()?;
    let json = String::from_utf8_lossy(&output.stdout[..]);
    Ok(serde_json::from_str(&json)?)
}

use std::{
    io::Read,
    sync::mpsc,
    thread,
};

pub fn listen(device_id: u32) -> mpsc::Receiver<CounterValues>
{
    let (sender, receiver) = mpsc::channel();
    {
        let sender = sender.clone();
        thread::spawn(move || {
            let listener =
                snmptool(&[&format!("{}", device_id), "listen"]).expect("spawning listener");
            let stdout = listener.stdout.expect("getting stdout handler from listener");
            let mut buf: Vec<u8> = Vec::with_capacity(64);
            for byte in stdout.bytes() {
                let byte = byte.unwrap();
                if byte == 0x0a {
                    let json = String::from_utf8_lossy(&buf[..]);
                    let counter = serde_json::from_str(&json).expect("deserializing CounterValues");
                    sender.send(counter).expect("sending CounterValues");
                    buf = Vec::with_capacity(64);
                }
                buf.push(byte);
            }
        });
    }
    receiver
}

#[cfg(test)]
pub mod tests
{
    pub const DEVICE_ID: u32 = 42719;
    use crate::tool::*;
    #[test]
    pub fn test_status()
    {
        let result = status(DEVICE_ID);
        println!("{:?}", result);
        assert!(result.is_ok())
    }

    #[test]
    pub fn test_counter()
    {
        let result = counter(DEVICE_ID);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_listen()
    {
        let receiver = listen(DEVICE_ID);
        loop {
            let counter = receiver.recv().expect("receiving counter values");
            println!("counter: {:?}", counter);
        }
    }
}
