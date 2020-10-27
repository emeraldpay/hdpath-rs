#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    HighBitIsSet,
    InvalidLength(usize),
    InvalidPurpose(u32),
    InvalidStructure,
    InvalidFormat
}