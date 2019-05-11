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

pub fn wake(ip: &str) { let _wake = snmptool(&[&format!("{}", ip), "wake"]); }

pub fn sleep(ip: &str) { let _sleep = snmptool(&[&format!("{}", ip), "sleep"]); }

pub fn clear(ip: &str) -> io::Result<()> { let _clear = snmptool(&[&format!("{}", ip), "clear"])?; Ok(()) }

pub fn status(ip: &str) -> Result<StatusValues, SnmpToolError>
{
    let output = snmptool(&[&format!("{}", ip), "status"])?.wait_with_output()?;
    let json = String::from_utf8_lossy(&output.stdout[..]);
    Ok(serde_json::from_str(&json)?)
}

pub fn counter(ip: &str) -> Result<CounterValues, SnmpToolError>
{
    let output = snmptool(&[ip, "counter"])?.wait_with_output()?;
    let json = String::from_utf8_lossy(&output.stdout[..]);
    Ok(serde_json::from_str(&json)?)
}
