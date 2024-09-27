use super::commands::{DbHandler, NotEnoughArgsError};
use crate::data_types::{data_types::Savable, table::Table};
use std::rc::Rc;

impl DbHandler {
    pub fn handle_set<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {        
        self.save(
            cmd.next().ok_or(NotEnoughArgsError {})?.to_owned(),
            Rc::new(cmd.next().ok_or(NotEnoughArgsError {})?.to_owned())
        )?;


        Ok(b"done".to_vec())
    }

    pub fn save(&mut self, key: String, value: Rc<dyn Savable>) -> Result<(), Box<dyn std::error::Error>> {
        let lock = self.data.lock();
        
        if let Ok(mut data) = lock {
            data.insert(key, value);
        } else {
            panic!("{:?}", lock);
        }
        
        Ok(())
    }

    pub fn handle_touch<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let name = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        let ty = cmd.next().ok_or( NotEnoughArgsError {})?;

        self.touch(name, match ty {
            "str" => 0,
            "table" => 1,
            _ => 0
        })?;

        Ok(b"done".to_vec())
    }

    pub fn touch(&mut self, name: String, ty: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.save(name,
            
            match ty {
            1 => Rc::new(Table::default()),
            _ => Rc::new(String::default())
        })?;

        Ok(())
    }
}