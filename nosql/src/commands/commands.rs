use core::fmt;
use std::fmt::Debug;
//use std::fs::File;
use std::sync::Mutex;
use crate::data_types::data_types::Savable;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::SplitWhitespace;


pub struct KeyNotFoundError {key: String}

impl fmt::Display for KeyNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key {} not found in database", self.key)
    }
}

impl Debug for KeyNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key {} not found in database", self.key)
    }
}

impl Debug for dyn Savable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for KeyNotFoundError {}

pub struct DbHandler {
    //dump_file: File,
    data: Mutex<HashMap<String, Rc<dyn Savable>>>
}

impl DbHandler {
    pub fn save(&mut self, key: String, value: Rc<dyn Savable>) -> Result<(), Box<dyn std::error::Error>> {
        let lock = self.data.lock();
        
        if let Ok(mut data) = lock {
            data.insert(key, value);
        } else {
            panic!("{:?}", lock);
        }
        
        Ok(())
    }

    pub fn load(&mut self, key: String) -> Result<Rc<dyn Savable>, Box<dyn std::error::Error>> {
        let lock = self.data.lock();

        if let Ok(data) = lock {
            if let Some(val) = data.get(&key) {
                Ok(val.clone())
            } else {
                Err(Box::new(KeyNotFoundError {key}))
            }
        } else {
            panic!("{:?}", lock);
        }
    }

    pub fn handle_command(&mut self, cmd: &mut SplitWhitespace) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let command = cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no command"))?;
    
        match command {
            "set" => self.handle_set(cmd),
            "get" => self.handle_get(cmd),
            _ => Ok(b"unknown command".to_vec())
        }
    }
    
    fn handle_set(&mut self, cmd: &mut SplitWhitespace) -> Result<Vec<u8>, Box<dyn std::error::Error>> {        
        self.save(
            cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no key"))?.to_owned(),
            Rc::new(cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no value"))?.to_owned())
        )?;


        Ok(b"done".to_vec())
    }
    
    fn handle_get(&mut self, cmd: &mut SplitWhitespace) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let bind = self.load(cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no key"))?.to_owned())?;
        let retr = bind.to_bin();

        Ok(retr.to_vec())
    }
}

impl Default for DbHandler {
    fn default() -> Self {
        DbHandler { data: Mutex::new(HashMap::new()) }
    }
}