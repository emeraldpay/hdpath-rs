use crate::{Purpose, CustomHDPath, Error, PathValue};
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShortHDPath {
    pub purpose: Purpose,
    pub coin_type: u32,
    pub account: u32,
    pub index: u32
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
        let value = CustomHDPath::try_from(value)?;
        ShortHDPath::try_from(value)
    }
}

impl ToString for ShortHDPath {
    fn to_string(&self) -> String {
        format!("m/{}'/{}'/{}'/{}",
                self.purpose.as_value().as_number(),
                self.coin_type,
                self.account,
                self.index
        )
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