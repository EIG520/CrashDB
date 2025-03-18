use crate::data_types::data_types::Savable;
use crate::data_types::table::Table;

impl Table {
    pub fn handle_list<'a>(&mut self, _: impl Iterator<Item = &'a str>) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
        let mut concated = String::from("");
        for str in self.data.keys() {concated = if concated.len() == 0 {str.to_owned()} else {format!("{concated}\n{str}")} }
        return Ok(concated.to_bin().to_vec());
    }
}