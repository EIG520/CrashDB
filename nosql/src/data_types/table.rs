use std::collections::HashMap;
use std::rc::Rc;

use crate::data_types::data_types::{Savable, Loadable};
use crate::commands::commands::DbHandler;

use super::data_types::DBDataType;

pub struct Table {
    data: HashMap<String, Rc<dyn Savable>>,
    bin_data: Vec<u8>
}

impl Savable for Table {
    fn to_bin(&self) -> &[u8] {
        &self.bin_data
    }
    fn signature(&self) -> u8 { 0 }
}

impl Loadable for Table {
    // not a chance I'm doing strings for table
    // TODO: ..implement this for real
    fn from_str(_: &str) -> Table { Table::default() }

    fn from_bin(b: &[u8]) -> Table {
        let mut table = Table::default();
        let mut idx = 0;

        loop {
            if idx >= b.len() {break;}

            let type_signature = b[idx];
            idx += 1;

            let size: usize = ((b[idx] as usize) << 24)
                + ((b[idx + 1] as usize) << 16)
                + ((b[idx + 2] as usize) << 8)
                + b[idx + 3] as usize;
            idx += 4;

            let mut key_bytes = vec![];
            while b[idx] != 0 {
                key_bytes.push(b[idx]);
                idx += 1;
            }
            idx += 1;
            let key = String::from_bin(&key_bytes);

            let value_bytes = &b[idx..(idx+size)];

            let value: Rc<dyn Savable> = match type_signature {
                0 => Rc::new(String::from_bin(&value_bytes)),
                1 => Rc::new(Table::from_bin(&value_bytes)),
                _ => Rc::new(String::from_bin(&value_bytes)),
            };

            table.insert(key, value);
        }

        table
    }
}

impl Default for Table {
    fn default() -> Self { Table {data: HashMap::new(), bin_data: vec![0]} }
}

impl DBDataType for Table {}

impl Table {
    pub fn insert(&mut self, key: String, value: Rc<dyn Savable>) {
        self.data.insert(key, value);
        // TODO: NOT THIS!!!!
        self.update_bin_data();
    }

    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
        self.update_bin_data();
    }

    pub fn update_bin_data(&mut self) {
        self.bin_data = vec![];

        for (key, val) in &self.data {
            self.bin_data.extend(DbHandler::kv_bits(key, val));
        }
    }
}