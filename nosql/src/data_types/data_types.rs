use super::table::Table;

pub trait Savable {
    fn to_bin(&self) -> &[u8];
    fn signature(&self) -> u8;
}

pub trait Loadable {
    fn from_str(s: &str) -> Self;
    fn from_bin(b: &[u8]) -> Self;
}

pub trait DBDataType : Savable + Loadable {}

pub enum SavableType {
    String(String),
    Table(Table),
}

impl Savable for SavableType {
    fn to_bin(&self) -> &[u8] {
        match self {
            SavableType::String(t) => {t.to_bin()},
            SavableType::Table(t) => {t.to_bin()}
        }
    }
    fn signature(&self) -> u8 {
        match self {
            SavableType::String(t) => {t.signature()},
            SavableType::Table(t) => {t.signature()}
        }
    }
}