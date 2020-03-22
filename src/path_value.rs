pub const FIRST_BIT: u32 = 0x80000000;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PathValue {
    Normal(u32),
    Hardened(u32)
}

impl PathValue {
    pub fn is_ok(value: u32) -> bool {
        value < FIRST_BIT
    }

    pub fn normal(value: u32) -> PathValue {
        if !PathValue::is_ok(value) {
            panic!("Raw hardened value passed")
        } else {
            PathValue::Normal(value)
        }
    }

    pub fn hardened(value: u32) -> PathValue {
        if !PathValue::is_ok(value) {
            panic!("Raw hardened value passed")
        } else {
            PathValue::Hardened(value)
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

#[cfg(test)]
mod tests {
    use super::*;

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