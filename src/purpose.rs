use std::cmp::Ordering;
use crate::{PathValue, Error};
use std::convert::TryFrom;
#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::{ChildNumber};

/// The purpose number, a first number in HD Path, which is supposed to be reference actual format. Supposed to be a hardened value
/// See [BIP-43](https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki)
#[derive(Debug, Clone, Eq, Hash)]
pub enum Purpose {
    None, //0'
    Pubkey, //44'
    ScriptHash, //49'
    Witness, //84'
    Custom(u32)
}

impl PartialOrd for Purpose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.as_value().to_raw() > other.as_value().to_raw() {
            Some(Ordering::Greater)
        } else if self.as_value().to_raw() == other.as_value().to_raw() {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for Purpose {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.as_value().to_raw() > other.as_value().to_raw() {
            Ordering::Greater
        } else if self.as_value().to_raw() == other.as_value().to_raw() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for Purpose {
    fn eq(&self, other: &Self) -> bool {
        self.as_value().to_raw() == other.as_value().to_raw()
    }
}

impl Purpose {
    pub fn as_value(&self) -> PathValue {
        let n = match self {
            Purpose::None => 0,
            Purpose::Pubkey => 44,
            Purpose::ScriptHash => 49,
            Purpose::Witness => 84,
            Purpose::Custom(n) => *n
        };
        PathValue::Hardened(n)
    }
}

impl TryFrom<u32> for Purpose {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            44 => Ok(Purpose::Pubkey),
            49 => Ok(Purpose::ScriptHash),
            84 => Ok(Purpose::Witness),
            n => if PathValue::is_ok(n) {
                Ok(Purpose::Custom(n))
            } else {
                Err(Error::HighBitIsSet)
            }
        }
    }
}

impl From<Purpose> for u32 {
    fn from(value: Purpose) -> Self {
        match value {
            Purpose::None => 0,
            Purpose::Pubkey => 44,
            Purpose::ScriptHash => 49,
            Purpose::Witness => 84,
            Purpose::Custom(n) => n.clone()
        }
    }
}

impl From<&Purpose> for u32 {
    fn from(value: &Purpose) -> Self {
        match value {
            Purpose::None => 0,
            Purpose::Pubkey => 44,
            Purpose::ScriptHash => 49,
            Purpose::Witness => 84,
            Purpose::Custom(n) => n.clone()
        }
    }
}

impl TryFrom<usize> for Purpose {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Purpose::try_from(value as u32)
    }
}

impl TryFrom<i32> for Purpose {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(Error::InvalidPurpose(0))
        }
        Purpose::try_from(value as u32)
    }
}

impl TryFrom<PathValue> for Purpose {
    type Error = Error;

    fn try_from(value: PathValue) -> Result<Self, Self::Error> {
        Purpose::try_from(value.as_number())
    }
}

#[cfg(feature = "with-bitcoin")]
impl From<Purpose> for ChildNumber {
    fn from(value: Purpose) -> Self {
        ChildNumber::from_hardened_idx(value.into()).unwrap()
    }
}

#[cfg(feature = "with-bitcoin")]
impl From<&Purpose> for ChildNumber {
    fn from(value: &Purpose) -> Self {
        ChildNumber::from_hardened_idx(value.into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    pub fn create_standard_purpose() {
        assert_eq!(Purpose::Pubkey, Purpose::try_from(44 as u32).unwrap());
        assert_eq!(Purpose::Pubkey, Purpose::try_from(44 as usize).unwrap());
        assert_eq!(Purpose::Pubkey, Purpose::try_from(44).unwrap());

        assert_eq!(Purpose::ScriptHash, Purpose::try_from(49).unwrap());
        assert_eq!(Purpose::Witness, Purpose::try_from(84).unwrap());
    }

    #[test]
    pub fn create_custom_purpose() {
        assert_eq!(Purpose::Custom(101), Purpose::try_from(101).unwrap());
    }

    #[test]
    pub fn compare() {
        assert!(Purpose::None < Purpose::Witness);
        assert!(Purpose::None < Purpose::Pubkey);
        assert!(Purpose::Pubkey < Purpose::Witness);
        assert!(Purpose::ScriptHash < Purpose::Witness);
        assert!(Purpose::Custom(0) < Purpose::Witness);
        assert!(Purpose::Custom(100) > Purpose::Witness);
        assert!(Purpose::Custom(50) > Purpose::Pubkey);
    }

    #[test]
    pub fn order() {
        let mut values = [
            Purpose::Witness, Purpose::None, Purpose::Pubkey, Purpose::ScriptHash, Purpose::Pubkey,
            Purpose::Custom(44), Purpose::Custom(84), Purpose::Custom(50), Purpose::Custom(1000)
        ];
        values.sort();
        assert_eq!(
            [
                Purpose::None, Purpose::Pubkey, Purpose::Pubkey, Purpose::Custom(44), Purpose::ScriptHash,
                Purpose::Custom(50), Purpose::Witness,
                Purpose::Custom(84),  Purpose::Custom(1000)
            ],
            values
        )
    }

}