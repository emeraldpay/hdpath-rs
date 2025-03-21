use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    HighBitIsSet,
    InvalidLength(usize),
    InvalidPurpose(u32),
    InvalidStructure,
    InvalidFormat
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HighBitIsSet => write!(f, "High bit is set"),
            Error::InvalidLength(len) => write!(f, "Invalid length: {}", len),
            Error::InvalidPurpose(purpose) => write!(f, "Invalid purpose: {}", purpose),
            Error::InvalidStructure => write!(f, "Invalid structure"),
            Error::InvalidFormat => write!(f, "Invalid format")
        }
    }
}

impl std::error::Error for Error {}