use crate::{Purpose, CustomHDPath, Error, PathValue, StandardHDPath};
use std::convert::TryFrom;
#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::{ChildNumber, DerivationPath};
use std::str::FromStr;
use crate::traits::HDPath;


/// Account-only HD Path for [BIP-44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki),
/// [BIP-49](https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki), [BIP-84](https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki)
/// and similar.
///
/// It's not supposed to be used to derive actual addresses, but only to build other path based on this
///
/// Represents `m/purpose'/coin_type'/account'/x/x`, like `m/44'/0'/0'/x/x`.
///
/// # Create new
/// ```
/// use hdpath::{AccountHDPath, Purpose};
///
/// //creates path m/84'/0'/0'
/// let hd_account = AccountHDPath::new(Purpose::Witness, 0, 0);
/// ```
/// # Parse string
/// ```
/// use hdpath::{AccountHDPath};
/// # use std::str::FromStr;
///
/// //creates path m/84'/0'/0'
/// let hd_account = AccountHDPath::from_str("m/84'/0'/0'").unwrap();
/// ```
///
/// Internal type and index can be explicitly market as unused (which is the default format for converting it into a string).
///
/// ```
/// use hdpath::{AccountHDPath};
/// # use std::str::FromStr;
///
/// //creates path m/84'/0'/0'
/// let hd_account = AccountHDPath::from_str("m/84'/0'/0'/x/x").unwrap();
/// ```
///
/// # Create actial path
/// ```
/// use hdpath::{AccountHDPath, Purpose, StandardHDPath};
/// # use std::str::FromStr;
///
/// let hd_account = AccountHDPath::from_str("m/84'/0'/0'").unwrap();
/// // gives hd path m/84'/0'/0'/0/4
/// let hd_path: StandardHDPath = hd_account.address_at(0, 4).unwrap();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AccountHDPath {
    purpose: Purpose,
    coin_type: u32,
    account: u32,
}

impl AccountHDPath {

    pub fn new(purpose: Purpose, coin_type: u32, account: u32) -> AccountHDPath {
        match Self::try_new(purpose, coin_type, account) {
            Ok(path) => path,
            Err(err) => panic!("Invalid {}: {}", err.0, err.1)
        }
    }

    pub fn try_new(purpose: Purpose, coin_type: u32, account: u32) -> Result<AccountHDPath, (String, u32)> {
        if let Purpose::Custom(n) = purpose {
            if !PathValue::is_ok(n) {
                return Err(("purpose".to_string(), n));
            }
        }
        if !PathValue::is_ok(coin_type) {
            return Err(("coin_type".to_string(), coin_type));
        }
        if !PathValue::is_ok(account) {
            return Err(("account".to_string(), account));
        }
        Ok(AccountHDPath {
            purpose,
            coin_type,
            account,
        })
    }

    /// Derive path to an address withing this account path
    /// ```
    /// # use hdpath::{AccountHDPath, Purpose, StandardHDPath};
    /// # use std::convert::TryFrom;
    /// let hd_account = AccountHDPath::try_from("m/84'/0'/0'").unwrap();
    /// // gives hd path m/84'/0'/0'/0/4
    /// let hd_path: StandardHDPath = hd_account.address_at(0, 4).unwrap();
    /// ```
    ///
    /// Return error `(field_name, invalid_value)` if the field has an incorrect value.
    /// It may happed if change or index are in _hardened_ space.
    pub fn address_at(&self, change: u32, index: u32) -> Result<StandardHDPath, (String, u32)> {
        StandardHDPath::try_new(
            self.purpose.clone(),
            self.coin_type,
            self.account,
            change,
            index
        )
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
}

impl HDPath for AccountHDPath {
    fn len(&self) -> u8 {
        3
    }

    fn get(&self, pos: u8) -> Option<PathValue> {
        match pos {
            0 => Some(self.purpose.as_value()),
            1 => Some(PathValue::Hardened(self.coin_type)),
            2 => Some(PathValue::Hardened(self.account)),
            _ => None
        }
    }
}

impl From<&StandardHDPath> for AccountHDPath {
    fn from(value: &StandardHDPath) -> Self {
        AccountHDPath::new(
            value.purpose().clone(),
            value.coin_type(),
            value.account(),
        )
    }
}

impl From<StandardHDPath> for AccountHDPath {
    fn from(value: StandardHDPath) -> Self {
        AccountHDPath::new(
            value.purpose().clone(),
            value.coin_type(),
            value.account(),
        )
    }
}

impl TryFrom<CustomHDPath> for AccountHDPath {
    type Error = Error;

    fn try_from(value: CustomHDPath) -> Result<Self, Self::Error> {
        if value.0.len() < 3 {
            return Err(Error::InvalidLength(value.0.len()))
        }
        if let Some(PathValue::Hardened(p)) = value.0.get(0) {
            let purpose = Purpose::try_from(*p)?;
            if let Some(PathValue::Hardened(coin_type)) = value.0.get(1) {
                if let Some(PathValue::Hardened(account)) = value.0.get(2) {
                    return Ok(AccountHDPath {
                        purpose,
                        coin_type: *coin_type,
                        account: *account,
                    })
                }
            }
            Err(Error::InvalidStructure)
        } else {
            Err(Error::InvalidStructure)
        }
    }
}

impl TryFrom<&str> for AccountHDPath
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AccountHDPath::from_str(value)
    }
}

impl FromStr for AccountHDPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = if s.ends_with("/x/x") {
            &s[0..s.len() - 4]
        } else {
            s
        };
        let value = CustomHDPath::from_str(clean)?;
        AccountHDPath::try_from(value)
    }
}

impl std::fmt::Display for AccountHDPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m/{}'/{}'/{}'/x/x",
               self.purpose.as_value().as_number(),
               self.coin_type,
               self.account,
        )
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&AccountHDPath> for Vec<ChildNumber> {
    fn from(value: &AccountHDPath) -> Self {
        let result = [
            ChildNumber::from_hardened_idx(value.purpose.as_value().as_number())
                .expect("Purpose is not Hardened"),
            ChildNumber::from_hardened_idx(value.coin_type)
                .expect("Coin Type is not Hardened"),
            ChildNumber::from_hardened_idx(value.account)
                .expect("Account is not Hardened"),
        ];
        return result.to_vec();
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<AccountHDPath> for Vec<ChildNumber> {
    fn from(value: AccountHDPath) -> Self {
        Vec::<ChildNumber>::from(&value)
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<AccountHDPath> for DerivationPath {
    fn from(value: AccountHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(&value))
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&AccountHDPath> for DerivationPath {
    fn from(value: &AccountHDPath) -> Self {
        DerivationPath::from(Vec::<ChildNumber>::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn create_try_from_string() {
        let hd_account = AccountHDPath::try_from("m/84'/0'/5'");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Witness, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type);
        assert_eq!(5, hd_account.account);
    }

    #[test]
    fn create_from_string() {
        let hd_account = AccountHDPath::from_str("m/84'/0'/5'");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Witness, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type);
        assert_eq!(5, hd_account.account);
    }

    #[test]
    fn create_from_acc_string() {
        let hd_account = AccountHDPath::from_str("m/84'/0'/5'/x/x");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Witness, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type);
        assert_eq!(5, hd_account.account);
    }

    #[test]
    fn create_from_string_sh() {
        let hd_account = AccountHDPath::try_from("m/49'/0'/5'");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::ScriptHash, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type());
        assert_eq!(5, hd_account.account());
    }

    #[test]
    fn create_from_string_pubkey() {
        let hd_account = AccountHDPath::try_from("m/44'/0'/5'");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Pubkey, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type);
        assert_eq!(5, hd_account.account);
    }

    #[test]
    fn create_from_string_custom() {
        let hd_account = AccountHDPath::try_from("m/218'/0'/5'");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Custom(218), hd_account.purpose);
        assert_eq!(0, hd_account.coin_type());
        assert_eq!(5, hd_account.account());
    }

    #[test]
    fn create_from_full_string() {
        let hd_account = AccountHDPath::try_from("m/84'/0'/5'/0/101");
        assert!(hd_account.is_ok());
        let hd_account = hd_account.unwrap();
        assert_eq!(Purpose::Witness, hd_account.purpose);
        assert_eq!(0, hd_account.coin_type());
        assert_eq!(5, hd_account.account());
    }

    #[test]
    fn to_string() {
        let hd_account = AccountHDPath::try_from("m/84'/0'/5'/0/101").unwrap();
        assert_eq!("m/84'/0'/5'/x/x", hd_account.to_string());
    }

    #[test]
    fn create_change_address() {
        let hd_account = AccountHDPath::try_from("m/84'/0'/0'").unwrap();
        let hd_path = hd_account.address_at(1, 3).expect("address create");
        assert_eq!(
            StandardHDPath::try_from("m/84'/0'/0'/1/3").unwrap(),
            hd_path
        );
    }

    #[test]
    fn create_receive_address() {
        let hd_account = AccountHDPath::try_from("m/84'/0'/0'").unwrap();
        let hd_path = hd_account.address_at(0, 15).expect("address create");
        assert_eq!(
            StandardHDPath::try_from("m/84'/0'/0'/0/15").unwrap(),
            hd_path
        );
    }

    #[test]
    fn convert_from_full() {
        let hd_path = StandardHDPath::from_str("m/84'/0'/0'/0/15").unwrap();
        let hd_account = AccountHDPath::from(&hd_path);
        assert_eq!(AccountHDPath::from_str("m/84'/0'/0'").unwrap(), hd_account);

        let hd_path = StandardHDPath::from_str("m/84'/0'/3'/0/0").unwrap();
        let hd_account = AccountHDPath::from(&hd_path);
        assert_eq!(AccountHDPath::from_str("m/84'/0'/3'").unwrap(), hd_account);

        let hd_path = StandardHDPath::from_str("m/44'/1'/1'/0/0").unwrap();
        let hd_account = AccountHDPath::from(&hd_path);
        assert_eq!(AccountHDPath::from_str("m/44'/1'/1'").unwrap(), hd_account);
    }
}

#[cfg(all(test, feature = "with-bitcoin"))]
mod tests_with_bitcoin {
    use super::*;
    use std::convert::TryFrom;
    use bitcoin::bip32::ChildNumber;

    #[test]
    pub fn convert_to_childnumbers() {
        let hdpath = AccountHDPath::try_from("m/44'/60'/2'/0/3581").unwrap();
        let children: Vec<ChildNumber> = hdpath.into();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], ChildNumber::from_hardened_idx(44).unwrap());
        assert_eq!(children[1], ChildNumber::from_hardened_idx(60).unwrap());
        assert_eq!(children[2], ChildNumber::from_hardened_idx(2).unwrap());
    }

}