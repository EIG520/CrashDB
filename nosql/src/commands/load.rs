use super::commands::NotEnoughArgsError;
use crate::data_types::data_types::{Savable, SavableType};
use crate::data_types::table::Table;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Debug, Display};
use core::fmt;

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

impl std::error::Error for KeyNotFoundError {}

impl Table {
    pub fn handle_get<'a>(&mut self, mut cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let bind = self.load(cmd.next().ok_or(GetError::NotEnoughArgsError(NotEnoughArgsError {}))?.to_owned())?;
        let bind2 = bind.borrow();
        let retr = bind2.to_string_bin();

        Ok(retr.to_vec())
    }

    pub fn load(&mut self, key: String) -> Result<Rc<RefCell<SavableType>>, GetError> {
        if let Some(val) = self.data.get(&key) {
            Ok(val.clone())
        } else {
            Err(GetError::KeyNotFoundError(KeyNotFoundError {key}))
        }
    }
}

#[derive(Debug)]
pub enum GetError {
    KeyNotFoundError(KeyNotFoundError),
    NotEnoughArgsError(NotEnoughArgsError)
}

impl Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetError::KeyNotFoundError(e) => Display::fmt(&e, f),
            GetError::NotEnoughArgsError(e) => Display::fmt(&e, f)
        }
    }
}

impl std::error::Error for GetError {}