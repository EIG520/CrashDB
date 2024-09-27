use super::commands::{DbHandler, NotEnoughArgsError};
use crate::data_types::data_types::Savable;

impl DbHandler {
    pub fn handle_open<'a>(&mut self, mut cmd: impl Iterator<Item=&'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut dir = cmd.next().ok_or(NotEnoughArgsError {})?.to_owned().to_bin().to_vec();
        // add a null byte so that we know it is a response from open
        dir.push(0);

        Ok(dir)
    }
}