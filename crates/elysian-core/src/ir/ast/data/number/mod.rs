use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{Abs, Acos, Atan, Atan2, Max, Min, Mix, Sign};

use crate::ir::ast::Expr;

use super::{Struct, Value};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

impl Number {
    pub fn literal(self) -> Expr {
        Expr::Literal(Value::Number(self))
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

impl Add<Struct> for Number {
    type Output = Struct;

    fn add(self, mut rhs: Struct) -> Self::Output {
        for (_, value) in rhs.members.iter_mut() {
            *value = Value::from(self.clone()) + value.clone();
        }
        rhs
    }
}

impl Sub<Struct> for Number {
    type Output = Struct;

    fn sub(self, mut rhs: Struct) -> Self::Output {
        for (_, value) in rhs.members.iter_mut() {
            *value = Value::from(self.clone()) - value.clone();
        }
        rhs
    }
}

impl Mul<Struct> for Number {
    type Output = Struct;

    fn mul(self, mut rhs: Struct) -> Self::Output {
        for (_, value) in rhs.members.iter_mut() {
            *value = Value::from(self.clone()) * value.clone();
        }
        rhs
    }
}

impl Div<Struct> for Number {
    type Output = Struct;

    fn div(self, mut rhs: Struct) -> Self::Output {
        for (_, value) in rhs.members.iter_mut() {
            *value = Value::from(self.clone()) / value.clone();
        }
        rhs
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

impl Atan2 for Number {
    fn atan2(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a.atan2(b)),
            _ => panic!("Invalid Atan2"),
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

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Value::Number(value)
    }
}

impl From<Value> for Number {
    fn from(value: Value) -> Self {
        let Value::Number(n) = value else {
            panic!("Value {value:#?} is not a Number")
        };

        n
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
