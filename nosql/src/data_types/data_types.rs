pub use super::table::Table;
pub use super::int::Int;
use strum_macros::EnumString;

pub trait Savable {
    fn to_bin(&self) -> &[u8];
    fn to_string_bin(&self) -> Vec<u8>;
    fn signature(&self) -> u8;
}

pub trait Loadable {
    fn from_str(s: &str) -> Self;
    fn from_bin(b: &[u8]) -> Self;
}

pub trait DBDataType : Savable + Loadable {}

#[derive(Clone, EnumString)]
pub enum SavableType {
    #[strum(serialize = "str", serialize = "string")]
    String(String),
    #[strum(serialize = "table")]
    Table(Table),
    #[strum(serialize = "int")]
    Int(Int)
}

impl Savable for SavableType {
    fn to_bin(&self) -> &[u8] {
        match self {
            SavableType::String(t) => {t.to_bin()},
            SavableType::Table(t) => {t.to_bin()},
            SavableType::Int(t) => {t.to_bin()}
        }
    }
    fn signature(&self) -> u8 {
        match self {
            SavableType::String(t) => {t.signature()},
            SavableType::Table(t) => {t.signature()},
            SavableType::Int(t) => {t.signature()}
        }
    }
    fn to_string_bin(&self) -> Vec<u8> {
        match self {
            SavableType::String(t) => {t.to_string_bin()},
            SavableType::Table(t) => {t.to_string_bin()},
            SavableType::Int(t) => {t.to_string_bin()}
        }
    }
}

impl SavableType {
    pub fn from_string(s: String, typ: u8) -> Self {
        match typ {
            0 => SavableType::String(s),
            1 => SavableType::Table(Table::from_str(&s)),
            2 => SavableType::Int(Int::from_str(&s)),
            _ => SavableType::String(s)
        }
    }
}