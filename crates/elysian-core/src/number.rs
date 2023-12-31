use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use elysian_math::{
    Abs, Acos, Asin, Atan, Atan2, Clamp, Cos, Max, Min, Mix, Round, Sign, Sin, Tan,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    UInt(u64),
    SInt(i64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::UInt(n) => write!(f, "{n:}"),
            Number::SInt(n) => write!(f, "{n:}"),
            Number::Float(n) => write!(f, "{n:}"),
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::UInt(a), Number::UInt(b)) => Number::UInt(a + b),
            (Number::SInt(a), Number::SInt(b)) => Number::SInt(a + b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            n => panic!("Invalid Add {n:#?}"),
        }
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::UInt(a), Number::UInt(b)) => Number::UInt(a - b),
            (Number::SInt(a), Number::SInt(b)) => Number::SInt(a - b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
            _ => panic!("Invalid Sub"),
        }
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::UInt(a), Number::UInt(b)) => Number::UInt(a * b),
            (Number::SInt(a), Number::SInt(b)) => Number::SInt(a * b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
            _ => panic!("Invalid Mul"),
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::UInt(a), Number::UInt(b)) => Number::UInt(a / b),
            (Number::SInt(a), Number::SInt(b)) => Number::SInt(a / b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
            _ => panic!("Invalid Div"),
        }
    }
}

impl Rem<Number> for Number {
    type Output = Number;

    fn rem(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::UInt(a), Number::UInt(b)) => Number::UInt(a.rem_euclid(b)),
            (Number::SInt(a), Number::SInt(b)) => Number::SInt(a.rem_euclid(b)),
            (Number::Float(a), Number::Float(b)) => Number::Float(a.rem_euclid(b)),
            _ => panic!("Invalid Mod"),
        }
    }
}

impl Mix for Number {
    type T = Number;

    fn mix(self, to: Self, t: Self::T) -> Self {
        match (self, to, t) {
            (Number::Float(a), Number::Float(b), Number::Float(t)) => a.mix(b, t).into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Clamp for Number {
    fn clamp(self, min: Self, max: Self) -> Self {
        match (self, min, max) {
            (Number::UInt(t), Number::UInt(min), Number::UInt(max)) => t.clamp(min, max).into(),
            (Number::SInt(t), Number::SInt(min), Number::SInt(max)) => t.clamp(min, max).into(),
            (Number::Float(t), Number::Float(min), Number::Float(max)) => t.clamp(min, max).into(),
            _ => panic!("Invalid Clamp"),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Number::SInt(i) => Number::SInt(-i),
            Number::Float(f) => Number::Float(-f),
            _ => panic!("Invalid Neg"),
        }
    }
}

impl Abs for Number {
    fn abs(self) -> Self {
        match self {
            Number::SInt(i) => Number::SInt(i.abs()),
            Number::Float(f) => Number::Float(f.abs()),
            _ => panic!("Invalid Abs"),
        }
    }
}

impl Sign for Number {
    fn sign(self) -> Self {
        match self {
            Number::SInt(i) => Number::SInt(i.sign()),
            Number::Float(f) => Number::Float(f.sign()),
            _ => panic!("Invalid Sign"),
        }
    }
}

impl Round for Number {
    fn round(self) -> Self {
        match self {
            Number::Float(f) => Number::Float(f.round()),
            _ => panic!("Invalid Round"),
        }
    }
}

impl Min for Number {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::SInt(a), Number::SInt(b)) => Min::min(a, b).into(),
            (Number::Float(a), Number::Float(b)) => Number::Float(a.min(b)),
            _ => panic!("Invalid Min"),
        }
    }
}

impl Max for Number {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::SInt(a), Number::SInt(b)) => Max::max(a, b).into(),
            (Number::Float(a), Number::Float(b)) => Number::Float(a.max(b)),
            _ => panic!("Invalid Max"),
        }
    }
}

impl Sin for Number {
    fn sin(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.sin()),
            _ => panic!("Invalid Sin"),
        }
    }
}

impl Cos for Number {
    fn cos(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.cos()),
            _ => panic!("Invalid Cos"),
        }
    }
}

impl Tan for Number {
    fn tan(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.tan()),
            _ => panic!("Invalid Tan"),
        }
    }
}

impl Asin for Number {
    fn asin(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.asin()),
            _ => panic!("Invalid Asin"),
        }
    }
}

impl Acos for Number {
    fn acos(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.acos()),
            _ => panic!("Invalid Atan2"),
        }
    }
}

impl Atan for Number {
    fn atan(self) -> Self {
        match self {
            Number::Float(a) => Number::Float(a.atan()),
            _ => panic!("Invalid Atan"),
        }
    }
}

impl Atan2 for Number {
    fn atan2(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a.atan2(b)),
            _ => panic!("Invalid Atan2"),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::UInt(value.into())
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::UInt(value.into())
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::UInt(value.into())
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::UInt(value.into())
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::SInt(value.into())
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::SInt(value.into())
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::SInt(value.into())
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::SInt(value.into())
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value.into())
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value.into())
    }
}

impl From<Number> for u8 {
    fn from(value: Number) -> Self {
        match value {
            Number::UInt(n) => n as u8,
            n => panic!("Number {n:#?} is not a u8"),
        }
    }
}

impl From<Number> for u16 {
    fn from(value: Number) -> Self {
        match value {
            Number::UInt(n) => n as u16,
            n => panic!("Number {n:#?} is not a u16"),
        }
    }
}

impl From<Number> for u32 {
    fn from(value: Number) -> Self {
        match value {
            Number::UInt(n) => n as u32,
            n => panic!("Number {n:#?} is not a u32"),
        }
    }
}

impl From<Number> for u64 {
    fn from(value: Number) -> Self {
        match value {
            Number::UInt(n) => n,
            n => panic!("Number {n:#?} is not a u64"),
        }
    }
}

impl From<Number> for i8 {
    fn from(value: Number) -> Self {
        match value {
            Number::SInt(n) => n as i8,
            n => panic!("Number {n:#?} is not a i8"),
        }
    }
}

impl From<Number> for i16 {
    fn from(value: Number) -> Self {
        match value {
            Number::SInt(n) => n as i16,
            n => panic!("Number {n:#?} is not a i16"),
        }
    }
}

impl From<Number> for i32 {
    fn from(value: Number) -> Self {
        match value {
            Number::SInt(n) => n as i32,
            n => panic!("Number {n:#?} is not a i32"),
        }
    }
}

impl From<Number> for i64 {
    fn from(value: Number) -> Self {
        match value {
            Number::SInt(n) => n,
            n => panic!("Number {n:#?} is not a i64"),
        }
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        match value {
            Number::Float(n) => n as f32,
            n => panic!("Number {n:#?} is not an f32"),
        }
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        match value {
            Number::Float(n) => n,
            n => panic!("Number {n:#?} is not an f64"),
        }
    }
}
