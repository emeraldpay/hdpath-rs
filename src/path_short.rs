use crate::{Purpose, CustomHDPath, Error, PathValue};
use std::convert::TryFrom;
#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::{ChildNumber, DerivationPath};
use std::str::FromStr;
use crate::traits::HDPath;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ShortHDPath {
    pub purpose: Purpose,
    pub coin_type: u32,
    pub account: u32,
    pub index: u32
}

impl HDPath for ShortHDPath {
    fn len(&self) -> u8 {
        4
    }

    fn get(&self, pos: u8) -> Option<PathValue> {
        match pos {
            0 => Some(self.purpose.as_value()),
            1 => Some(PathValue::Hardened(self.coin_type)),
            2 => Some(PathValue::Hardened(self.account)),
            3 => Some(PathValue::Normal(self.index)),
            _ => None
        }
    }
}

impl TryFrom<CustomHDPath> for ShortHDPath {
    type Error = Error;

    fn try_from(value: CustomHDPath) -> Result<Self, Self::Error> {
        if value.0.len() != 4 {
            return Err(Error::InvalidLength(value.0.len()))
        }
        if let Some(PathValue::Hardened(p)) = value.0.get(0) {
            let purpose = Purpose::try_from(*p)?;
            if let Some(PathValue::Hardened(coin_type)) = value.0.get(1) {
                if let Some(PathValue::Hardened(account)) = value.0.get(2) {
                    if let Some(PathValue::Normal(index)) = value.0.get(3) {
                        return Ok(ShortHDPath {
                            purpose,
                            coin_type: *coin_type,
                            account: *account,
                            index: *index
                        })
                    }
                }
            }
            Err(Error::InvalidStructure)
        } else {
            Err(Error::InvalidStructure)
        }
    }
}

impl TryFrom<&str> for ShortHDPath
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ShortHDPath::from_str(value)
    }
}

impl FromStr for ShortHDPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = CustomHDPath::from_str(s)?;
        ShortHDPath::try_from(value)
    }
}

impl fmt::Display for ShortHDPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "m/{}'/{}'/{}'/{}",
               self.purpose.as_value().as_number(),
               self.coin_type,
               self.account,
               self.index
        )
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&ShortHDPath> for Vec<ChildNumber> {
    fn from(value: &ShortHDPath) -> Self {
        let result = [
            ChildNumber::from_hardened_idx(value.purpose.as_value().as_number())
                .expect("Purpose is not Hardened"),
            ChildNumber::from_hardened_idx(value.coin_type)
                .expect("Coin Type is not Hardened"),
            ChildNumber::from_hardened_idx(value.account)
                .expect("Account is not Hardened"),
            ChildNumber::from_normal_idx(value.index)
                .expect("Index is Hardened"),
        ];
        return result.to_vec();
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<ShortHDPath> for Vec<ChildNumber> {
    fn from(value: ShortHDPath) -> Self {
        Vec::<ChildNumber>::from(&value)
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<ShortHDPath> for DerivationPath {
    fn from(value: ShortHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(&value))
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&ShortHDPath> for DerivationPath {
    fn from(value: &ShortHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_string_short() {
        assert_eq!("m/44'/60'/0'/0", ShortHDPath {
            purpose: Purpose::Pubkey,
            coin_type: 60,
            account: 0,
            index: 0
        }.to_string());
        assert_eq!("m/44'/61'/0'/0", ShortHDPath {
            purpose: Purpose::Pubkey,
            coin_type: 61,
            account: 0,
            index: 0
        }.to_string());
        assert_eq!("m/101'/61'/0'/0", ShortHDPath {
            purpose: Purpose::Custom(101),
            coin_type: 61,
            account: 0,
            index: 0
        }.to_string());
    }

    #[test]
    pub fn to_string_short_all() {
        let paths = vec![
            "m/44'/0'/0'/0",
            "m/44'/60'/0'/1",
            "m/44'/60'/160720'/0",
            "m/44'/60'/160720'/101",
        ];
        for p in paths {
            assert_eq!(p, ShortHDPath::try_from(p).unwrap().to_string())
        }
    }
}

#[cfg(all(test, feature = "with-bitcoin"))]
mod tests_with_bitcoin {
    use super::*;
    use std::convert::TryFrom;
    use bitcoin::bip32::ChildNumber;

    #[test]
    pub fn convert_to_childnumbers() {
        let hdpath = ShortHDPath::try_from("m/44'/60'/2'/100").unwrap();
        let childs: Vec<ChildNumber> = hdpath.into();
        assert_eq!(childs.len(), 4);
        assert_eq!(childs[0], ChildNumber::from_hardened_idx(44).unwrap());
        assert_eq!(childs[1], ChildNumber::from_hardened_idx(60).unwrap());
        assert_eq!(childs[2], ChildNumber::from_hardened_idx(2).unwrap());
        assert_eq!(childs[3], ChildNumber::from_normal_idx(100).unwrap());
    }

}