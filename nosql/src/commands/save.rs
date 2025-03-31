use super::commands::NotEnoughArgsError;
use crate::data_types::{data_types::{Loadable, Savable, SavableType}, int::Int, table::Table};
use std::{cell::RefCell, rc::Rc};

impl Table {
    pub fn handle_set<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {        
        let key = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        
        let string_val = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        let mut typ = 0;

        if let Ok(ty) = self.load(key.clone()) {
            typ = ty.borrow().signature();
        }
        
        let val = match typ {
            0 => SavableType::String(string_val),
            1 => SavableType::Table(Table::from_str(&string_val)),
            2 => SavableType::Int(Int::from_str(&string_val)), 
            _ => {SavableType::String(string_val)}
        };

        self.save(
            key,
            Rc::new(RefCell::new(val))
        )?;


        Ok(b"done".to_vec())
    }

    pub fn save(&mut self, key: String, value: Rc<RefCell<SavableType>>) -> Result<(), Box<dyn std::error::Error>> {
        self.insert(key, value);       
        Ok(())
    }

    pub fn handle_touch<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let name = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        let ty = cmd.next().ok_or( NotEnoughArgsError {})?;

        self.touch(name, match ty {
            "str" => 0,
            "table" => 1,
            "int" => 2,
            // default to string
            _ => 0
        })?;

        Ok(b"done".to_vec())
    }

    pub fn touch(&mut self, name: String, ty: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.save(name, Rc::new(RefCell::new(match ty {
            0 => SavableType::String(String::default()),
            1 => SavableType::Table(Table::default()),
            2 => SavableType::Int(Int::default()),
            // default to string
            _ => SavableType::String(String::default())
        })))?;

        Ok(())
    }
}