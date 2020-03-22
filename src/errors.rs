#[derive(Debug)]
pub enum Error {
    HighBitIsSet,
    InvalidLength(usize),
    InvalidPurpose(u32),
    InvalidStructure,
    InvalidFormat
}