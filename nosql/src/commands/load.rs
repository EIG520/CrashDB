use super::commands::DbHandler;
use crate::data_types::data_types::Savable;
use std::rc::Rc;
use std::fmt::Debug;
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

impl DbHandler {
    pub fn handle_get<'a>(&mut self, mut cmd: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + '_>> {
        let bind = self.load(cmd.next().ok_or(std::io::Error::new(std::io::ErrorKind::AddrInUse, "no key"))?.to_owned())?;
        let retr = bind.to_bin();

        Ok(retr.to_vec())
    }

    fn load(&mut self, key: String) -> Result<Rc<dyn Savable>, Box<dyn std::error::Error + '_>> {
        let lock = self.data.lock();

        if let Some(val) = lock?.get(&key) {
            Ok(val.clone())
        } else {
            Err(Box::new(KeyNotFoundError {key}))
        }
    }
}