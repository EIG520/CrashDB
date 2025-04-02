use super::commands::NotEnoughArgsError;
use crate::data_types::{data_types::{Savable, SavableType}, table::Table};
use std::{cell::RefCell, rc::Rc, str::FromStr};

impl Table {
    pub fn handle_set<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {        
        let key = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        
        let string_val = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned();
        let mut typ = 0;

        if let Ok(ty) = self.load(key.clone()) {
            typ = ty.borrow().signature();
        }
        
        let val = SavableType::from_string(string_val, typ);

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

        self.save(name, Rc::new(RefCell::new(SavableType::from_str(ty)?)))?;

        Ok(b"done".to_vec())
    }

    // pub fn touch(&mut self, name: String, ty: u8) -> Result<(), Box<dyn std::error::Error>> {
    //     self.save(name, Rc::new(RefCell::new(            
    //         match ty {
    //         0 => SavableType::String(String::default()),
    //         1 => SavableType::Table(Table::default()),
    //         2 => SavableType::Int(Int::default()),
    //         // default to string
    //         _ => SavableType::String(String::default())
    //         }
    //     )))?;

    //     Ok(())
    // }
}