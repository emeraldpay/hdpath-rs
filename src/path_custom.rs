use crate::{PathValue, Error};
use std::convert::TryFrom;
#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::{ChildNumber, DerivationPath};
use std::str::FromStr;
use crate::traits::HDPath;

/// A custom HD Path, that can be any length and contain any Hardened and non-Hardened values in
/// any order. Direct implementation for [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#The_default_wallet_layout)
///
/// If you need just standard type of HD Path like `m/44'/0'/0'/0/0` use [`StandardHDPath`](struct.StandardHDPath.html) instead.
///
/// # Usage
///
/// ## Parse string
/// ```
/// use hdpath::CustomHDPath;
/// # use std::convert::TryFrom;
///
/// let hdpath = CustomHDPath::try_from("m/1'/2'/3/4/5'/6'/7").unwrap();
/// let hdpath = CustomHDPath::try_from("m/44'/0'/1'/0/0").unwrap();
/// //also support uppercase notation
/// let hdpath = CustomHDPath::try_from("M/44H/0H/1H/0/0").unwrap();
/// ```
/// ## Direct create
/// ```
/// use hdpath::{CustomHDPath, PathValue};
///
/// let hdpath = CustomHDPath::try_new(vec![
///    PathValue::hardened(44), PathValue::hardened(0), PathValue::hardened(1),
///    PathValue::normal(0), PathValue::normal(0)
/// ]).unwrap();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustomHDPath(pub Vec<PathValue>);

impl CustomHDPath {

    /// Create a new HD Path.
    ///
    /// Returns error only if provided vector is too large, i.e. more than 255 elements (since BIP-32
    /// says about ability to encode depth in a single byte).
    pub fn try_new(values: Vec<PathValue>) -> Result<CustomHDPath, Error> {
        if values.len() > 0xff {
            Err(Error::InvalidLength(values.len()))
        } else {
            Ok(CustomHDPath(values))
        }
    }
}

impl HDPath for CustomHDPath {
    fn len(&self) -> u8 {
        self.0.len() as u8
    }

    fn get(&self, pos: u8) -> Option<PathValue> {
        self.0.get(pos as usize).map(|a| a.clone())
    }
}

impl TryFrom<&str> for CustomHDPath {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CustomHDPath::from_str(value)
    }
}

impl std::convert::From<&dyn HDPath> for CustomHDPath {
    fn from(value: &dyn HDPath) -> Self {
        let mut path = Vec::with_capacity(value.len() as usize);
        for i in 0..value.len() {
            path.push(value.get(i).expect("no-path-element"));
        }
        CustomHDPath(path)
    }
}

impl std::fmt::Display for CustomHDPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m")?;
        for pv in self.0.iter() {
            write!(f, "/{}", pv)?;
        }
        Ok(())
    }
}

impl FromStr for CustomHDPath {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        const STATE_EXPECT_NUM: usize = 0;
        const STATE_READING_NUM: usize = 1;
        const STATE_READ_MARKER: usize = 2;

        let chars = value.as_bytes();
        if chars.len() < 2 {
            return Err(Error::InvalidFormat)
        }
        if chars[0] != 'm' as u8 && chars[0] != 'M' as u8 {
            return Err(Error::InvalidFormat)
        }
        if chars[1] != '/' as u8 {
            return Err(Error::InvalidFormat)
        }
        let mut keys: Vec<PathValue> = Vec::new();
        let mut pos = 2;
        let mut num: u32 = 0;
        let mut state = STATE_EXPECT_NUM;
        while chars.len() > pos {
            match chars[pos] {
                39 | 72 => { // (') apostrophe or H
                    if state != STATE_READING_NUM {
                        return Err(Error::InvalidFormat)
                    }
                    if !PathValue::is_ok(num) {
                        return Err(Error::InvalidFormat)
                    }
                    keys.push(PathValue::hardened(num));
                    state = STATE_READ_MARKER;
                    num = 0;
                },
                47 => { // slash
                    if state == STATE_READING_NUM {
                        if !PathValue::is_ok(num) {
                            return Err(Error::InvalidFormat)
                        }
                        keys.push(PathValue::normal(num));
                    } else if state != STATE_READ_MARKER {
                        return Err(Error::InvalidFormat)
                    }
                    state = STATE_EXPECT_NUM;
                    num = 0;
                },
                48..=57 => { //number
                    if state == STATE_EXPECT_NUM {
                        state = STATE_READING_NUM
                    } else if state != STATE_READING_NUM {
                        return Err(Error::InvalidFormat)
                    }
                    num = num * 10 + (chars[pos] - 48) as u32;
                },
                _ => {
                    return Err(Error::InvalidFormat)
                }
            }
            pos += 1;
            if chars.len() == pos && state == 1 {
                if !PathValue::is_ok(num) {
                    return Err(Error::InvalidFormat)
                }
                keys.push(PathValue::normal(num));
            }
        }
        if state == STATE_EXPECT_NUM {
            //finished with slash
            Err(Error::InvalidFormat)
        } else if keys.is_empty() {
            Err(Error::InvalidStructure)
        } else {
            Ok(CustomHDPath(keys))
        }
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&CustomHDPath> for Vec<ChildNumber> {
    fn from(value: &CustomHDPath) -> Self {
        let mut result: Vec<ChildNumber> = Vec::with_capacity(value.0.len());
        for item in value.0.iter() {
            result.push(ChildNumber::from(item.to_raw()))
        }
        return result;
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<CustomHDPath> for Vec<ChildNumber> {
    fn from(value: CustomHDPath) -> Self {
        Vec::<ChildNumber>::from(&value)
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<CustomHDPath> for DerivationPath {
    fn from(value: CustomHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(&value))
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&CustomHDPath> for DerivationPath {
    fn from(value: &CustomHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StandardHDPath;

    #[test]
    pub fn to_string() {
        assert_eq!(
            CustomHDPath::try_from("m/44'/0'/0'/0/0").unwrap().to_string(),
            "m/44'/0'/0'/0/0".to_string()
        );

        assert_eq!(
            CustomHDPath::try_from("m/84'/1'/2'/3/4").unwrap().to_string(),
            "m/84'/1'/2'/3/4".to_string()
        );

        assert_eq!(
            CustomHDPath::try_from("m/1'").unwrap().to_string(),
            "m/1'".to_string()
        );

        assert_eq!(
            CustomHDPath::try_from("m/44'/0'/1'/2/3/4'/5/67'/8'/910").unwrap().to_string(),
            "m/44'/0'/1'/2/3/4'/5/67'/8'/910".to_string()
        );
    }

    #[test]
    pub fn try_from_common() {
        let act = CustomHDPath::try_from("m/44'/0'/0'/0/0").unwrap();
        act.0[0].as_number();
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Hardened(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(4).unwrap());
    }

    #[test]
    pub fn try_from_common_trait() {
        let source = StandardHDPath::from_str("m/84'/0'/1'/2/3").unwrap();
        let act = CustomHDPath::from(source.to_trait());
        assert_eq!(
            CustomHDPath::try_from("m/84'/0'/1'/2/3").unwrap(), act
        );
    }

    #[test]
    pub fn try_from_bignum() {
        let act = CustomHDPath::try_from("m/44'/12'/345'/6789/101112").unwrap();
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Hardened(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(12), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(345), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(6789), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(101112), act.0.get(4).unwrap());
    }

    #[test]
    pub fn try_from_long() {
        let act = CustomHDPath::try_from("m/44'/0'/1'/2/3/4'/5/67'/8'/910").unwrap();
        assert_eq!(10, act.0.len());
        assert_eq!(&PathValue::Hardened(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(1), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(2), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(3), act.0.get(4).unwrap());
        assert_eq!(&PathValue::Hardened(4), act.0.get(5).unwrap());
        assert_eq!(&PathValue::Normal(5), act.0.get(6).unwrap());
        assert_eq!(&PathValue::Hardened(67), act.0.get(7).unwrap());
        assert_eq!(&PathValue::Hardened(8), act.0.get(8).unwrap());
        assert_eq!(&PathValue::Normal(910), act.0.get(9).unwrap());
    }

    #[test]
    pub fn try_from_all_hardened() {
        let act = CustomHDPath::try_from("m/44'/0'/0'/0'/1'").unwrap();
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Hardened(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Hardened(1), act.0.get(4).unwrap());
    }

    #[test]
    pub fn try_from_all_normal() {
        let act = CustomHDPath::try_from("m/44/0/0/0/1").unwrap();
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Normal(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(1), act.0.get(4).unwrap());
    }

    #[test]
    pub fn try_from_other_format() {
        let act = CustomHDPath::try_from("M/44H/0H/0H/1/5").unwrap();
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Hardened(44), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(1), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(5), act.0.get(4).unwrap());
    }

    #[test]
    pub fn error_on_invalid_path() {
        let paths = vec![
            "", "1", "m44",
            "m/", "m/44/", "m/44/0/", "m/44''/0/0/0/1", "m/44/H0/0/0/1",
        ];
        for p in paths {
            assert!(CustomHDPath::try_from(p).is_err(), "test: {}", p);
        }
    }

    #[test]
    pub fn fail_incorrect_hardened() {
        let custom = CustomHDPath::try_from("m/2147483692'/0'/0'/0/0");
        assert!(custom.is_err());
    }

    #[test]
    pub fn cannot_create_too_long() {
        let mut path = Vec::with_capacity(0xff + 1);
        for _i in 0..path.capacity() {
            path.push(PathValue::Normal(1));
        }
        let custom = CustomHDPath::try_new(path);
        assert!(custom.is_err());
        assert_eq!(Error::InvalidLength(256), custom.expect_err("not error"));
    }
}

#[cfg(all(test, feature = "with-bitcoin"))]
mod tests_with_bitcoin {
    use super::*;
    use std::convert::TryFrom;
    use bitcoin::bip32::ChildNumber;

    #[test]
    pub fn convert_to_childnumbers() {
        let hdpath = CustomHDPath::try_from("m/44'/15'/2'/0/35/81/0").unwrap();
        let childs: Vec<ChildNumber> = hdpath.into();
        assert_eq!(childs.len(), 7);
        assert_eq!(childs[0], ChildNumber::from_hardened_idx(44).unwrap());
        assert_eq!(childs[1], ChildNumber::from_hardened_idx(15).unwrap());
        assert_eq!(childs[2], ChildNumber::from_hardened_idx(2).unwrap());
        assert_eq!(childs[3], ChildNumber::from_normal_idx(0).unwrap());
        assert_eq!(childs[4], ChildNumber::from_normal_idx(35).unwrap());
        assert_eq!(childs[5], ChildNumber::from_normal_idx(81).unwrap());
        assert_eq!(childs[6], ChildNumber::from_normal_idx(0).unwrap());
    }

}