//use std::fs::File;
use crate::data_types::data_types::{Savable, SavableType};
use crate::data_types::table::Table;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt;
use std::rc::Rc;

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
    fn apply<'a, A>(handler: &mut DbHandler, cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub struct DbHandler {
    pub dump_path: String,
    pub data: Rc<RefCell<SavableType>>
}

impl DbHandler {
    // DB-level commands (checked first)
    pub fn handle_command<'a>(&'a mut self, first: &'a str, cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + '_>> {    
        return match first {
            "dump" => self.handle_dump(cmd),
            "hawk" => Ok(b"tuah".to_vec()),
            _ => Ok(b"unknown command".to_vec())
        }
    }
}

impl Table {
    // Table-level commands
    pub fn handle_command<'a>(&mut self, first: &'a str, cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + '_>> {        
        return match first {
            "set" => self.handle_set(cmd),
            "get" => self.handle_get(cmd),
            "touch" => self.handle_touch(cmd),
            _ => Ok(b"unknown command".to_vec())
        };
    }
}

impl Default for DbHandler {
    fn default() -> Self {
        DbHandler { data: Rc::new(RefCell::new(SavableType::Table(Table::default()))), dump_path: "dump.dat".to_owned() }
    }
}