use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{Abs, Max, Min, Sign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SInt {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Display for SInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SInt::I8(i) => write!(f, "{i:}"),
            SInt::I16(i) => write!(f, "{i:}"),
            SInt::I32(i) => write!(f, "{i:}"),
            SInt::I64(i) => write!(f, "{i:}"),
        }
    }
}

impl Add for SInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a + b),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a + b),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a + b),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a + b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Sub for SInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a - b),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a - b),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a - b),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a - b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Mul for SInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a * b),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a * b),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a * b),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a * b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Div for SInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a / b),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a / b),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a / b),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a / b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Neg for SInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            SInt::I8(i) => SInt::I8(-i),
            SInt::I16(i) => SInt::I16(-i),
            SInt::I32(i) => SInt::I32(-i),
            SInt::I64(i) => SInt::I64(-i),
        }
    }
}

impl Abs for SInt {
    fn abs(self) -> Self {
        match self {
            SInt::I8(i) => SInt::I8(i.abs()),
            SInt::I16(i) => SInt::I16(i.abs()),
            SInt::I32(i) => SInt::I32(i.abs()),
            SInt::I64(i) => SInt::I64(i.abs()),
        }
    }
}

impl Sign for SInt {
    fn sign(self) -> Self {
        match self {
            SInt::I8(i) => SInt::I8(i.sign()),
            SInt::I16(i) => SInt::I16(i.sign()),
            SInt::I32(i) => SInt::I32(i.sign()),
            SInt::I64(i) => SInt::I64(i.sign()),
        }
    }
}

impl Min for SInt {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a.min(b)),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a.min(b)),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a.min(b)),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a.min(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Max for SInt {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (SInt::I8(a), SInt::I8(b)) => SInt::I8(a.max(b)),
            (SInt::I16(a), SInt::I16(b)) => SInt::I16(a.max(b)),
            (SInt::I32(a), SInt::I32(b)) => SInt::I32(a.max(b)),
            (SInt::I64(a), SInt::I64(b)) => SInt::I64(a.max(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<i8> for SInt {
    fn from(value: i8) -> Self {
        SInt::I8(value)
    }
}

impl From<i16> for SInt {
    fn from(value: i16) -> Self {
        SInt::I16(value)
    }
}

impl From<i32> for SInt {
    fn from(value: i32) -> Self {
        SInt::I32(value)
    }
}

impl From<i64> for SInt {
    fn from(value: i64) -> Self {
        SInt::I64(value)
    }
}

impl From<SInt> for i8 {
    fn from(value: SInt) -> Self {
        match value {
            SInt::I8(i) => i,
            _ => panic!("Invalid conversion")
        }
    }
}

impl From<SInt> for i16 {
    fn from(value: SInt) -> Self {
        match value {
            SInt::I16(i) => i,
            _ => panic!("Invalid conversion")
        }
    }
}

impl From<SInt> for i32 {
    fn from(value: SInt) -> Self {
        match value {
            SInt::I32(i) => i,
            _ => panic!("Invalid conversion")
        }
    }
}

impl From<SInt> for i64 {
    fn from(value: SInt) -> Self {
        match value {
            SInt::I64(i) => i,
            _ => panic!("Invalid conversion")
        }
    }
}
