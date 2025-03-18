use crate::data_types::data_types::{Savable, Loadable};
use super::data_types::DBDataType;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Int {
    val: i64,
    valby: [u8; 8]
}

impl Savable for Int {
    fn to_bin(&self) -> &[u8] { 
        return &self.valby;
    }
    fn signature(&self) -> u8 { 2 }
    fn to_string_bin(&self) -> Vec<u8> {
        format!("{}", self.val).as_bytes().to_vec()
    }
}

impl Add for Int {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut new = Self::Output{val: self.val + other.val, valby: [0;8]};
        new.update_by();

        return new;
    }
}

impl Loadable for Int {
    fn from_str(s: &str) -> Self {
        let mut i = Int {val: s.parse::<i64>().unwrap(), valby: [0; 8]};
        i.update_by();
        i
    }
    fn from_bin(b: &[u8]) -> Self {
        let mut i = Int { val:
              (b[0] as i64) << 56
              + (b[1] as i64) << 48
              + (b[2] as i64) << 40
              + (b[3] as i64) << 32
              + (b[4] as i64) << 24
              + (b[5] as i64) << 16
              + (b[6] as i64) << 8
              + (b[7] as i64),
            valby: [0; 8]
        };
        i.update_by();
        i
    }
}

impl Int {
    fn update_by(&mut self) {
        let mask = 0b11111111;
        self.valby = [((self.val >> 56) & mask) as u8, ((self.val >> 48) & mask) as u8, ((self.val >> 40) & mask) as u8, ((self.val >> 32) & mask) as u8, ((self.val >> 24) & mask) as u8, ((self.val >> 16) & mask) as u8, ((self.val >> 8) & mask) as u8, ((self.val) & mask) as u8];
    }
}

impl Default for Int {
    fn default() -> Self {
        Int {val: 0, valby: [0; 8]}
    }
}

impl From<i64> for Int {
    fn from(val: i64) -> Self {
        let mut new = Self {val, valby: [0; 8]};
        new.update_by();

        new
    }
}

impl DBDataType for Int {}