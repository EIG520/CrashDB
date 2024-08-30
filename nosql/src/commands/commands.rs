//use std::fs::File;
use std::sync::Mutex;
use crate::data_types::data_types::Savable;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::SplitWhitespace;
use std::fmt::Debug;
use std::fmt;

impl Debug for dyn Savable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct NotEnoughArgsError {}

impl fmt::Display for NotEnoughArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough arguments in command")
    }
}

impl Debug for NotEnoughArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough arguments in command")
    }
}

impl std::error::Error for NotEnoughArgsError {}

pub trait Command {
    fn apply(handler: &mut DbHandler, cmd: &mut SplitWhitespace) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub struct DbHandler {
    //dump_file: File,
    pub data: Mutex<HashMap<String, Rc<dyn Savable>>>
}

impl DbHandler {

    pub fn handle_command(&mut self, cmd: &mut SplitWhitespace) -> Result<Vec<u8>, Box<dyn std::error::Error + '_>> {
        let command = cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no command"))?;
    
        match command {
            "set" => self.handle_set(cmd),
            "get" => self.handle_get(cmd),
            _ => Ok(b"unknown command".to_vec())
        }
    }
}

impl Default for DbHandler {
    fn default() -> Self {
        DbHandler { data: Mutex::new(HashMap::new()) }
    }
}