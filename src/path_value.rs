#[cfg(feature = "with-bitcoin")]
use bitcoin::bip32::ChildNumber;

pub const FIRST_BIT: u32 = 0x80000000;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PathValue {
    Normal(u32),
    Hardened(u32),
}

impl PathValue {
    pub fn is_ok(value: u32) -> bool {
        value < FIRST_BIT
    }

    pub fn try_normal(value: u32) -> Result<PathValue, ()> {
        if !PathValue::is_ok(value) {
            Err(())
        } else {
            Ok(PathValue::Normal(value))
        }
    }

    pub fn normal(value: u32) -> PathValue {
        if let Ok(result) = PathValue::try_normal(value) {
            result
        } else {
            panic!("Raw hardened value passed")
        }
    }

    pub fn try_hardened(value: u32) -> Result<PathValue, ()> {
        if !PathValue::is_ok(value) {
            Err(())
        } else {
            Ok(PathValue::Hardened(value))
        }
    }

    pub fn hardened(value: u32) -> PathValue {
        if let Ok(result) = PathValue::try_hardened(value) {
            result
        } else {
            panic!("Raw hardened value passed")
        }
    }

    pub fn from_raw(value: u32) -> PathValue {
        if value >= FIRST_BIT {
            PathValue::Hardened(value - FIRST_BIT)
        } else {
            PathValue::Normal(value)
        }
    }

    pub fn as_number(&self) -> u32 {
        match &self {
            PathValue::Normal(n) => *n,
            PathValue::Hardened(n) => *n
        }
    }

    pub fn to_raw(&self) -> u32 {
        match &self {
            PathValue::Normal(n) => *n,
            PathValue::Hardened(n) => *n + FIRST_BIT
        }
    }
}

#[cfg(feature = "with-bitcoin")]
impl From<PathValue> for ChildNumber {
    fn from(value: PathValue) -> Self {
        match value {
            PathValue::Hardened(i) => ChildNumber::from_hardened_idx(i).unwrap(),
            PathValue::Normal(i) => ChildNumber::from_normal_idx(i).unwrap(),
        }
    }
}

impl std::fmt::Display for PathValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathValue::Normal(n) => write!(f, "{}", n),
            PathValue::Hardened(n) => write!(f, "{}'", n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "with-bitcoin")]
    use bitcoin::bip32::ChildNumber;

    #[test]
    #[cfg(feature = "with-bitcoin")]
    fn convert_to_bitcoin() {
        let act: ChildNumber = PathValue::Normal(0).into();
        assert_eq!(ChildNumber::from_normal_idx(0).unwrap(), act);

        let act: ChildNumber = PathValue::Normal(1).into();
        assert_eq!(ChildNumber::from_normal_idx(1).unwrap(), act);

        let act: ChildNumber = PathValue::Normal(100).into();
        assert_eq!(ChildNumber::from_normal_idx(100).unwrap(), act);

        let act: ChildNumber = PathValue::Hardened(0).into();
        assert_eq!(ChildNumber::from_hardened_idx(0).unwrap(), act);

        let act: ChildNumber = PathValue::Hardened(1).into();
        assert_eq!(ChildNumber::from_hardened_idx(1).unwrap(), act);

        let act: ChildNumber = PathValue::Hardened(11).into();
        assert_eq!(ChildNumber::from_hardened_idx(11).unwrap(), act);
    }

    #[test]
    fn to_string_normal() {
        assert_eq!(PathValue::Normal(0).to_string(), "0");
        assert_eq!(PathValue::Normal(11).to_string(), "11");
    }

    #[test]
    fn to_string_hardened() {
        assert_eq!(PathValue::Hardened(0).to_string(), "0'");
        assert_eq!(PathValue::Hardened(1).to_string(), "1'");
    }

    #[test]
    fn display_normal() {
        assert_eq!(format!("{}", PathValue::Normal(0)), "0");
        assert_eq!(format!("{}", PathValue::Normal(11)), "11");
    }

    #[test]
    fn display_hardened() {
        assert_eq!(format!("{}", PathValue::Hardened(0)), "0'");
        assert_eq!(format!("{}", PathValue::Hardened(11)), "11'");
    }

    #[test]
    fn ok_for_small_values() {
        let values = vec![
            0u32, 1, 2, 3,
            100, 1000, 10000,
            0x80000000 - 1
        ];
        for value in values {
            assert!(PathValue::is_ok(value), "value: {}", value);
        }
    }

    #[test]
    fn not_ok_for_large_values() {
        let values = vec![
            0x80000000, 0x80000001,
            0xffffffff
        ];
        for value in values {
            assert!(!PathValue::is_ok(value), "value: {}", value);
        }
    }

    #[test]
    fn create_normal() {
        assert_eq!(PathValue::Normal(0), PathValue::normal(0));
        assert_eq!(PathValue::Normal(1), PathValue::normal(1));
        assert_eq!(PathValue::Normal(101), PathValue::normal(101));
    }

    #[test]
    fn create_hardened() {
        assert_eq!(PathValue::Hardened(0), PathValue::hardened(0));
        assert_eq!(PathValue::Hardened(1), PathValue::hardened(1));
        assert_eq!(PathValue::Hardened(101), PathValue::hardened(101));
    }

    #[test]
    #[should_panic]
    fn panic_on_invalid_normal() {
        PathValue::normal(0x80000001);
    }

    #[test]
    #[should_panic]
    fn panic_on_invalid_hardened() {
        PathValue::hardened(0x80000001);
    }

    #[test]
    fn create_from_raw_normal() {
        assert_eq!(PathValue::Normal(0), PathValue::from_raw(0));
        assert_eq!(PathValue::Normal(100), PathValue::from_raw(100));
        assert_eq!(PathValue::Normal(0xffffff), PathValue::from_raw(0xffffff));
    }

    #[test]
    fn create_from_raw_hardened() {
        assert_eq!(PathValue::hardened(0), PathValue::from_raw(0x80000000));
        assert_eq!(PathValue::hardened(1), PathValue::from_raw(0x80000001));
        assert_eq!(PathValue::hardened(44), PathValue::from_raw(0x8000002c));
    }

    #[test]
    fn to_raw_normal() {
        assert_eq!(0, PathValue::Normal(0).to_raw());
        assert_eq!(123, PathValue::Normal(123).to_raw());
    }

    #[test]
    fn to_raw_hardened() {
        assert_eq!(0x80000000, PathValue::Hardened(0).to_raw());
        assert_eq!(0x8000002c, PathValue::Hardened(44).to_raw());
    }

    #[test]
    fn as_number_normal() {
        assert_eq!(0, PathValue::Normal(0).as_number());
        assert_eq!(123, PathValue::Normal(123).as_number());
    }

    #[test]
    fn as_number_hardened() {
        assert_eq!(0, PathValue::Hardened(0).as_number());
        assert_eq!(123, PathValue::Hardened(123).as_number());
    }
}