pub struct VEField {
    pub label: String,
    pub value: Vec<u8>,
}

pub const CR: u8 = 13;
pub const LF: u8 = 10;
pub const TAB: u8 = 9;
pub const COLON: u8 = 58;
pub const A: u8 = 65;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VEError {
    Parse(String),
    HexBytes,
    NeedMoreData,
    UnknownCode,
}
