pub trait Savable {
    fn to_bin(&self) -> &[u8];
}

pub trait Loadable {
    fn from_str(s: &str) -> Self;
    fn from_bin(b: &[u8]) -> Self;
}

pub trait DBDataType : Savable + Loadable {}