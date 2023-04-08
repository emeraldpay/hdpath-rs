#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    HighBitIsSet,
    InvalidLength(usize),
    InvalidPurpose(u32),
    InvalidStructure,
    InvalidFormat
}