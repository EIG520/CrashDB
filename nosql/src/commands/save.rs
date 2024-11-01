use super::commands::NotEnoughArgsError;
use crate::data_types::{data_types::SavableType, table::Table};
use std::{cell::RefCell, rc::Rc};

impl Table {
    pub fn handle_set<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {        
        self.save(
            cmd.next().ok_or(NotEnoughArgsError {})?.to_owned(),
            Rc::new(RefCell::new(SavableType::String(cmd.next().ok_or(NotEnoughArgsError {})?.to_owned())))
        )?;


        Ok(b"done".to_vec())
    }

    pub fn save(&mut self, key: String, value: Rc<RefCell<SavableType>>) -> Result<(), Box<dyn std::error::Error>> {
        self.data.insert(key, value);       
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
        self.save(name, Rc::new(RefCell::new(match ty {
            1 => SavableType::Table(Table::default()),
            _ => SavableType::String(String::default())
        })))?;

        Ok(())
    }
}