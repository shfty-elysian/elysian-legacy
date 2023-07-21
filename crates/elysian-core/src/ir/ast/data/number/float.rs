use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{Abs, Max, Min, Mix, Sign};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::F32(v) => write!(f, "{v:}"),
            Float::F64(v) => write!(f, "{v:}"),
        }
    }
}

impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a + b),
            (Float::F64(a), Float::F64(b)) => Float::F64(a + b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a - b),
            (Float::F64(a), Float::F64(b)) => Float::F64(a - b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a * b),
            (Float::F64(a), Float::F64(b)) => Float::F64(a * b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a / b),
            (Float::F64(a), Float::F64(b)) => Float::F64(a / b),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Mix for Float {
    type T = Float;

    fn mix(self, to: Self, t: Self::T) -> Self {
        match (self, to, t) {
            (Float::F32(from), Float::F32(to), Float::F32(t)) => from.mix(to, t).into(),
            (Float::F64(from), Float::F64(to), Float::F64(t)) => from.mix(to, t).into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Float::F32(i) => Float::F32(-i),
            Float::F64(i) => Float::F64(-i),
        }
    }
}

impl Abs for Float {
    fn abs(self) -> Self {
        match self {
            Float::F32(i) => Float::F32(i.abs()),
            Float::F64(i) => Float::F64(i.abs()),
        }
    }
}

impl Sign for Float {
    fn sign(self) -> Self {
        match self {
            Float::F32(i) => Float::F32(i.sign()),
            Float::F64(i) => Float::F64(i.sign()),
        }
    }
}

impl Min for Float {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a.min(b)),
            (Float::F64(a), Float::F64(b)) => Float::F64(a.min(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Max for Float {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Float::F32(a), Float::F32(b)) => Float::F32(a.max(b)),
            (Float::F64(a), Float::F64(b)) => Float::F64(a.max(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::F32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::F64(value)
    }
}
