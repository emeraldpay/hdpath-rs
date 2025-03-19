use crate::{PathValue, CustomHDPath};
use byteorder::{BigEndian, WriteBytesExt};
#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::{ChildNumber, DerivationPath};

/// General trait for an HDPath.
/// Common implementations are [`StandardHDPath`], [`AccountHDPath`] and [`CustomHDPath`]
///
/// [`StandardHDPath`]: struct.StandardHDPath.html
/// [`AccountHDPath`]: struct.AccountHDPath.html
/// [`CustomHDPath`]: struct.CustomHDPath.html
pub trait HDPath {

    /// Size of the HD Path
    fn len(&self) -> u8;

    /// Get element as the specified position.
    /// The implementation must return `Some<PathValue>` for all values up to `len()`.
    /// And return `None` if the position if out of bounds.
    ///
    /// See [`PathValue`](enum.PathValue.html)
    fn get(&self, pos: u8) -> Option<PathValue>;

    /// Encode as bytes, where first byte is number of elements in path (always 5 for StandardHDPath)
    /// following by 4-byte BE values
    fn to_bytes(&self) -> Vec<u8> {
        let len = self.len();
        let mut buf = Vec::with_capacity(1 + 4 * (len as usize));
        buf.push(len);
        for i in 0..len {
            buf.write_u32::<BigEndian>(self.get(i)
                .expect(format!("No valut at {}", i).as_str())
                .to_raw()).unwrap();
        }
        buf
    }

    ///
    /// Get parent HD Path.
    /// Return `None` if the current path is empty (i.e. already at the top)
    fn parent(&self) -> Option<CustomHDPath> {
        if self.len() == 0 {
            return None
        }
        let len = self.len();
        let mut parent_hd_path = Vec::with_capacity(len as usize - 1);
        for i in 0..len - 1 {
            parent_hd_path.push(self.get(i).unwrap());
        }
        let parent_hd_path = CustomHDPath::try_new(parent_hd_path)
            .expect("No parent HD Path");
        Some(parent_hd_path)
    }

    ///
    /// Convert current to `CustomHDPath` structure
    fn as_custom(&self) -> CustomHDPath {
        let len = self.len();
        let mut path = Vec::with_capacity(len as usize);
        for i in 0..len {
            path.push(self.get(i).unwrap());
        }
        CustomHDPath::try_new(path).expect("Invalid HD Path")
    }

    ///
    /// Convert current to bitcoin lib type
    #[cfg(feature = "with-bitcoin")]
    fn as_bitcoin(&self) -> DerivationPath {
        let len = self.len();
        let mut path = Vec::with_capacity(len as usize);
        for i in 0..len {
            path.push(ChildNumber::from(self.get(i).unwrap()));
        }
        DerivationPath::from(path)
    }
}

#[cfg(feature = "with-bitcoin")]
impl std::convert::From<&dyn HDPath> for DerivationPath {
    fn from(value: &dyn HDPath) -> Self {
        let mut path = Vec::with_capacity(value.len() as usize);
        for i in 0..value.len() {
            path.push(ChildNumber::from(value.get(i).expect("no-path-element")));
        }
        DerivationPath::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StandardHDPath, AccountHDPath};
    use std::str::FromStr;

    impl StandardHDPath {
        pub fn to_trait(&self) -> &dyn HDPath {
            self
        }
    }

    #[test]
    fn get_parent_from_std() {
        let act = StandardHDPath::from_str("m/44'/0'/1'/1/2").unwrap();
        let parent = act.parent();
        assert!(parent.is_some());
        let parent = parent.unwrap();
        assert_eq!(
            "m/44'/0'/1'/1", parent.to_string()
        );
    }

    #[test]
    fn get_parent_twice() {
        let act = StandardHDPath::from_str("m/44'/0'/1'/1/2").unwrap();
        let parent = act.parent().unwrap().parent();
        assert!(parent.is_some());
        let parent = parent.unwrap();
        assert_eq!(
            "m/44'/0'/1'", parent.to_string()
        );
    }

    #[test]
    fn get_parent_from_account() {
        let act = AccountHDPath::from_str("m/84'/0'/1'").unwrap();
        let parent = act.parent();
        assert!(parent.is_some());
        let parent = parent.unwrap();
        assert_eq!(
            "m/84'/0'", parent.to_string()
        );
    }

    #[test]
    fn get_parent_from_custom() {
        let act = CustomHDPath::from_str("m/84'/0'/1'/0/16").unwrap();
        let parent = act.parent();
        assert!(parent.is_some());
        let parent = parent.unwrap();
        assert_eq!(
            "m/84'/0'/1'/0", parent.to_string()
        );
    }

    #[test]
    fn convert_account_to_custom() {
        let src = AccountHDPath::from_str("m/84'/0'/1'").unwrap();
        let act = src.as_custom();
        assert_eq!(CustomHDPath::from_str("m/84'/0'/1'").unwrap(), act);
    }

    #[test]
    fn convert_standard_to_custom() {
        let src = StandardHDPath::from_str("m/84'/0'/1'/0/2").unwrap();
        let act = src.as_custom();
        assert_eq!(CustomHDPath::from_str("m/84'/0'/1'/0/2").unwrap(), act);
    }
}

#[cfg(all(test, feature = "with-bitcoin"))]
mod tests_with_bitcoin {
    use crate::{StandardHDPath, HDPath};
    use std::str::FromStr;
    use bitcoin::bip32::{DerivationPath};

    #[test]
    fn convert_to_bitcoin() {
        let source = StandardHDPath::from_str("m/44'/0'/1'/1/2").unwrap();
        let act = DerivationPath::from(source.to_trait());
        assert_eq!(
            DerivationPath::from_str("m/44'/0'/1'/1/2").unwrap(),
            act
        )
    }

    #[test]
    fn convert_to_bitcoin_directly() {
        let source = StandardHDPath::from_str("m/44'/0'/1'/1/2").unwrap();
        let act = source.as_bitcoin();
        assert_eq!(
            DerivationPath::from_str("m/44'/0'/1'/1/2").unwrap(),
            act
        )
    }
}