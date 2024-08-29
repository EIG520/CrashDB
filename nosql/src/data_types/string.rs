use crate::data_types::data_types::{Savable, Loadable};

use super::data_types::DBDataType;

impl Savable for String {
    fn to_bin(&self) -> &[u8] { self.as_bytes() }
}

impl Loadable for String {
    fn from_str(s: &str) -> String { String::from(s) }
    fn from_bin(b: &[u8]) -> String { String::from_utf8(b.to_vec()).unwrap() }
}

impl DBDataType for String {}