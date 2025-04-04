use crate::data_types::{data_types::{Loadable, SavableType}, int::Int, table::Table};

use super::commands::NotEnoughArgsError;

impl Table {
    pub fn handle_inc<'a>(&mut self, mut cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let var = cmd.next().ok_or(NotEnoughArgsError {})?;
        let val = self.load(var.to_owned())?;
        let mut mval = val.borrow_mut();
        
        let res = match *mval {
            SavableType::Int(t) => {*mval = SavableType::Int(t+Int::from(1)); Ok(b"done".to_vec())},
            _ => Ok(b"inc can only be used on ints".to_vec())
        };

        res
    }

    pub fn handle_dec<'a>(&mut self, mut cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let var = cmd.next().ok_or(NotEnoughArgsError {})?;
        let val = self.load(var.to_owned())?;
        let mut mval = val.borrow_mut();
        
        let res = match *mval {
            SavableType::Int(t) => {*mval = SavableType::Int(t-Int::from(1)); Ok(b"done".to_vec())},
            _ => Ok(b"dec can only be used on ints".to_vec())
        };

        res
    }

    pub fn handle_add<'a>(&mut self, mut cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let var = cmd.next().ok_or(NotEnoughArgsError {})?;
        let val = self.load(var.to_owned())?;
        let toadd = Int::from_str(cmd.next().ok_or(NotEnoughArgsError {})?);
        let mut mval = val.borrow_mut();
        
        let res = match *mval {
            SavableType::Int(t) => {*mval = SavableType::Int(t+toadd); Ok(b"done".to_vec())},
            _ => Ok(b"inc can only be used on ints".to_vec())
        };

        res
    }
}