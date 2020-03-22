use crate::{Purpose, PathValue, Error, CustomHDPath};
use std::convert::TryFrom;

/// Standard HD Path for [BIP-44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki),
/// [BIP-49](https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki), [BIP-84](https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki)
/// and similar. For path as `m/purpose'/coin_type'/account'/change/address_index`, like `m/44'/0'/0'/0/0`.
///
/// # Create new
/// ```
/// use hdpath::{StandardHDPath, Purpose};
///
/// //creates path m/84'/0'/0'/0/0
/// let hdpath = StandardHDPath::new(Purpose::Witness, 0, 0, 0, 0);
/// ```
/// # Parse string
/// ```
/// use hdpath::{StandardHDPath, Purpose};
/// # use std::convert::TryFrom;
///
/// //creates path m/84'/0'/0'/0/0
/// let hdpath = StandardHDPath::try_from("m/84'/0'/0'/0/0");
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StandardHDPath {
    purpose: Purpose,
    coin_type: u32,
    account: u32,
    change: u32,
    index: u32
}

impl StandardHDPath {
    pub fn new(purpose: Purpose, coin_type: u32, account: u32, change: u32, index: u32) -> StandardHDPath {
        if let Purpose::Custom(n) = purpose {
            if !PathValue::is_ok(n) {
                panic!("Invalid purpose: {}", n);
            }
        }
        if !PathValue::is_ok(coin_type) {
            panic!("Invalid coin_type: {}", coin_type);
        }
        if !PathValue::is_ok(account) {
            panic!("Invalid account: {}", account);
        }
        if !PathValue::is_ok(change) {
            panic!("Invalid change: {}", change);
        }
        if !PathValue::is_ok(index) {
            panic!("Invalid index: {}", index);
        }
        StandardHDPath {
            purpose,
            coin_type,
            account,
            change,
            index
        }
    }

    pub fn purpose(&self) -> &Purpose {
        &self.purpose
    }

    pub fn coin_type(&self) -> u32 {
        self.coin_type
    }

    pub fn account(&self) -> u32 {
        self.account
    }

    pub fn change(&self) -> u32 {
        self.change
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Default for StandardHDPath {
    fn default() -> Self {
        StandardHDPath {
            purpose: Purpose::Pubkey,
            coin_type: 0,
            account: 0,
            change: 0,
            index: 0
        }
    }
}

impl TryFrom<CustomHDPath> for StandardHDPath {
    type Error = Error;

    fn try_from(value: CustomHDPath) -> Result<Self, Self::Error> {
        if value.0.len() != 5 {
            return Err(Error::InvalidLength(value.0.len()))
        }
        if let Some(PathValue::Hardened(p)) = value.0.get(0) {
            let purpose = Purpose::try_from(*p)?;
            if let Some(PathValue::Hardened(coin_type)) = value.0.get(1) {
                if let Some(PathValue::Hardened(account)) = value.0.get(2) {
                    if let Some(PathValue::Normal(change)) = value.0.get(3) {
                        if let Some(PathValue::Normal(index)) = value.0.get(4) {
                            return Ok(StandardHDPath::new(
                                purpose,
                                *coin_type,
                                *account,
                                *change,
                                *index
                            ))
                        }
                    }
                }
            }
            Err(Error::InvalidStructure)
        } else {
            Err(Error::InvalidStructure)
        }
    }
}

impl From<StandardHDPath> for CustomHDPath {
    fn from(value: StandardHDPath) -> Self {
        CustomHDPath(
            vec![
                value.purpose().as_value(),
                PathValue::Hardened(value.coin_type()),
                PathValue::Hardened(value.account()),
                PathValue::Normal(value.change()),
                PathValue::Normal(value.index()),
            ]
        )
    }
}

impl TryFrom<&str> for StandardHDPath
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = CustomHDPath::try_from(value)?;
        StandardHDPath::try_from(value)
    }
}

impl ToString for StandardHDPath {
    fn to_string(&self) -> String {
        format!("m/{}'/{}'/{}'/{}/{}",
                self.purpose().as_value().as_number(),
                self.coin_type(),
                self.account(),
                self.change(),
                self.index()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    pub fn from_custom() {
        let act = StandardHDPath::try_from(
            CustomHDPath::new(vec![
                PathValue::Hardened(49), PathValue::Hardened(0), PathValue::Hardened(1),
                PathValue::Normal(0), PathValue::Normal(5)
            ])
        ).unwrap();
        assert_eq!(
            StandardHDPath::new(Purpose::ScriptHash, 0, 1, 0, 5),
            act
        );

        let act = StandardHDPath::try_from(
            CustomHDPath::new(vec![
                PathValue::Hardened(44), PathValue::Hardened(60), PathValue::Hardened(1),
                PathValue::Normal(0), PathValue::Normal(0)
            ])
        ).unwrap();
        assert_eq!(
            StandardHDPath::new(Purpose::Pubkey, 60, 1, 0, 0),
            act
        );
    }

    #[test]
    pub fn from_standard_to_custom() {
        let standard = StandardHDPath::try_from("m/49'/0'/1'/0/5").unwrap();
        let act = CustomHDPath::from(standard);
        assert_eq!(5, act.0.len());
        assert_eq!(&PathValue::Hardened(49), act.0.get(0).unwrap());
        assert_eq!(&PathValue::Hardened(0), act.0.get(1).unwrap());
        assert_eq!(&PathValue::Hardened(1), act.0.get(2).unwrap());
        assert_eq!(&PathValue::Normal(0), act.0.get(3).unwrap());
        assert_eq!(&PathValue::Normal(5), act.0.get(4).unwrap());
    }

    #[test]
    pub fn to_standard_path_with_custom_purpose() {
        let act = StandardHDPath::try_from("m/101'/0'/1'/0/5").unwrap();
        assert_eq!(
            StandardHDPath::new(Purpose::Custom(101), 0, 1, 0, 5),
            act
        );
    }

    #[test]
    pub fn err_to_standard_path_not_hardened() {
        let paths = vec![
            "m/49/0'/1'/0/5",
            "m/49'/0/1'/0/5",
            "m/49'/0'/1/0/5",
            "m/49/0/1'/0/5",
        ];
        for p in paths {
            let custom = CustomHDPath::try_from(p).expect(format!("failed for: {}", p).as_str());
            assert!(StandardHDPath::try_from(custom).is_err(), "test: {}", p);
        }
    }

    #[test]
    pub fn to_string_standard_all() {
        let paths = vec![
            "m/44'/0'/0'/0/0",
            "m/44'/60'/0'/0/1",
            "m/44'/60'/160720'/0/2",
            "m/49'/0'/0'/0/0",
            "m/49'/0'/1'/0/5",
            "m/84'/0'/0'/0/0",
            "m/84'/0'/0'/1/120",
            "m/101'/0'/0'/1/101",
        ];
        for p in paths {
            assert_eq!(p, StandardHDPath::try_from(p).unwrap().to_string())
        }
    }

    #[test]
    pub fn order() {
        let path1 = StandardHDPath::new(Purpose::Pubkey, 0, 0, 0, 0);
        let path2 = StandardHDPath::new(Purpose::Pubkey, 0, 0, 0, 1);
        let path3 = StandardHDPath::new(Purpose::Pubkey, 0, 0, 1, 1);
        let path4 = StandardHDPath::new(Purpose::Witness, 0, 2, 0, 100);
        let path5 = StandardHDPath::new(Purpose::Witness, 0, 3, 0, 0);

        assert!(path1 < path2);
        assert!(path1 < path3);
        assert!(path1 < path4);
        assert!(path1 < path5);

        assert!(path2 > path1);
        assert!(path2 < path3);
        assert!(path2 < path4);
        assert!(path2 < path5);

        assert!(path3 > path1);
        assert!(path3 > path2);
        assert!(path3 < path4);
        assert!(path3 < path5);

        assert!(path4 > path1);
        assert!(path4 > path2);
        assert!(path4 > path3);
        assert!(path4 < path5);

        assert!(path5 > path1);
        assert!(path5 > path2);
        assert!(path5 > path3);
        assert!(path5 > path4);
    }

    #[test]
    pub fn order_with_custom_purpose() {
        let path1 = StandardHDPath::new(Purpose::Pubkey, 0, 0, 0, 0);
        let path2 = StandardHDPath::new(Purpose::Custom(60), 0, 0, 0, 0);
        let path3 = StandardHDPath::new(Purpose::Witness, 0, 0, 0, 0);

        assert!(path1 < path2);
        assert!(path2 < path3);
    }
}