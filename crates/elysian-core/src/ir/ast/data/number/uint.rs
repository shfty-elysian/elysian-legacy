use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use rust_gpu_bridge::{Max, Min};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Display for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UInt::U8(i) => write!(f, "{i:}"),
            UInt::U16(i) => write!(f, "{i:}"),
            UInt::U32(i) => write!(f, "{i:}"),
            UInt::U64(i) => write!(f, "{i:}"),
        }
    }
}

impl Add for UInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a + b),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a + b),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a + b),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a + b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Sub for UInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a - b),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a - b),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a - b),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a - b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Mul for UInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a * b),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a * b),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a * b),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a * b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Div for UInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a / b),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a / b),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a / b),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a / b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Min for UInt {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a.min(b)),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a.min(b)),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a.min(b)),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a.min(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Max for UInt {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (UInt::U8(a), UInt::U8(b)) => UInt::U8(a.max(b)),
            (UInt::U16(a), UInt::U16(b)) => UInt::U16(a.max(b)),
            (UInt::U32(a), UInt::U32(b)) => UInt::U32(a.max(b)),
            (UInt::U64(a), UInt::U64(b)) => UInt::U64(a.max(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<u8> for UInt {
    fn from(value: u8) -> Self {
        UInt::U8(value)
    }
}

impl From<u16> for UInt {
    fn from(value: u16) -> Self {
        UInt::U16(value)
    }
}

impl From<u32> for UInt {
    fn from(value: u32) -> Self {
        UInt::U32(value)
    }
}

impl From<u64> for UInt {
    fn from(value: u64) -> Self {
        UInt::U64(value)
    }
}

impl From<UInt> for u8 {
    fn from(value: UInt) -> Self {
        match value {
            UInt::U8(i) => i,
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<UInt> for u16 {
    fn from(value: UInt) -> Self {
        match value {
            UInt::U16(i) => i,
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<UInt> for u32 {
    fn from(value: UInt) -> Self {
        match value {
            UInt::U32(i) => i,
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<UInt> for u64 {
    fn from(value: UInt) -> Self {
        match value {
            UInt::U64(i) => i,
            _ => panic!("Invalid conversion"),
        }
    }
}
