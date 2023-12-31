use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Rem, Sub},
};

use elysian_math::{
    glam::{Vec2, Vec3, Vec4},
    Abs, Acos, Asin, Atan, Atan2, Clamp, Cos, Dot, Length, Max, Min, Mix, Normalize, Round, Sign,
    Sin, Tan,
};

use crate::module::StructIdentifier;
use elysian_core::number::Number;

use super::{
    Struct, MATRIX2, MATRIX3, MATRIX4, VECTOR2, VECTOR3, VECTOR4, W, W_AXIS_4, X, X_AXIS_2,
    X_AXIS_3, X_AXIS_4, Y, Y_AXIS_2, Y_AXIS_3, Y_AXIS_4, Z, Z_AXIS_3, Z_AXIS_4,
};

/// Concrete value
#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    Struct(Struct),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{b:}"),
            Value::Number(n) => write!(f, "{n:}"),
            Value::Struct(s) => write!(f, "{s:}"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Struct(a), Value::Struct(b)) => a.partial_cmp(b),
            _ => panic!("Invalid PartialOrd"),
        }
    }
}

impl Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a + b).into(),
            (Value::Number(a), Value::Struct(b)) => (a + b).into(),
            (Value::Struct(a), Value::Number(b)) => (a + b).into(),
            (Value::Struct(a), Value::Struct(b)) => (a + b).into(),
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Self;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a - b).into(),
            (Value::Number(a), Value::Struct(b)) => (a - b).into(),
            (Value::Struct(a), Value::Number(b)) => (a - b).into(),
            (Value::Struct(a), Value::Struct(b)) => (a - b).into(),
            _ => panic!("Invalid Sub"),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a * b).into(),
            (Value::Number(a), Value::Struct(b)) => (a * b).into(),
            (Value::Struct(a), Value::Number(b)) => (a * b).into(),
            (Value::Struct(a), Value::Struct(b)) => (a * b).into(),
            t => panic!("Invalid Mul {t:#?}"),
        }
    }
}

impl Div<Value> for Value {
    type Output = Self;

    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a / b).into(),
            (Value::Number(a), Value::Struct(b)) => (a / b).into(),
            (Value::Struct(a), Value::Number(b)) => (a / b).into(),
            (Value::Struct(a), Value::Struct(b)) => (a / b).into(),
            _ => panic!("Invalid Div"),
        }
    }
}

impl Rem<Value> for Value {
    type Output = Self;

    fn rem(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a % b).into(),
            (Value::Struct(a), Value::Struct(b)) => (a % b).into(),
            (a, b) => panic!("Invalid Mod {a:?} % {b:?}"),
        }
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a & b),
            _ => panic!("Invalid BitAnd"),
        }
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a | b),
            _ => panic!("Invalid BitOr"),
        }
    }
}

impl Mix for Value {
    type T = Value;

    fn mix(self, to: Self, t: Self::T) -> Self {
        let Value::Number(t) = t else {
            panic!("T is not a Number");
        };

        match (self, to) {
            (Value::Number(a), Value::Number(b)) => a.mix(b, t).into(),
            (Value::Struct(a), Value::Struct(b)) => a.mix(b, t).into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Clamp for Value {
    fn clamp(self, min: Self, max: Self) -> Self {
        match (self, min, max) {
            (Value::Number(t), Value::Number(min), Value::Number(max)) => t.clamp(min, max).into(),
            (Value::Struct(t), Value::Struct(min), Value::Struct(max)) => t.clamp(min, max).into(),
            _ => panic!("Invalid Clamp"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => (-n).into(),
            Value::Struct(v) => (-v).into(),
            _ => panic!("Invalid Neg"),
        }
    }
}

impl Abs for Value {
    fn abs(self) -> Self {
        match self {
            Value::Number(n) => n.abs().into(),
            Value::Struct(v) => v.abs().into(),
            _ => panic!("Invalid Abs"),
        }
    }
}

impl Sign for Value {
    fn sign(self) -> Self {
        match self {
            Value::Number(n) => n.sign().into(),
            Value::Struct(v) => v.sign().into(),
            _ => panic!("Invalid Sign"),
        }
    }
}

impl Round for Value {
    fn round(self) -> Self {
        match self {
            Value::Number(n) => n.round().into(),
            Value::Struct(v) => v.round().into(),
            _ => panic!("Invalid Sign"),
        }
    }
}

impl Length for Value {
    type Output = Self;

    fn length(self) -> Self::Output {
        match self {
            Value::Number(n) => n.abs().into(),
            Value::Struct(v) => v.length().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Normalize for Value {
    fn normalize(self) -> Self {
        match self {
            Value::Number(n) => n.sign().into(),
            Value::Struct(v) => v.normalize().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Dot for Value {
    type Output = Self;

    fn dot(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a * b).into(),
            (Value::Struct(a), Value::Struct(b)) => a.dot(b).into(),
            _ => panic!("Invalid Dot"),
        }
    }
}

impl Min for Value {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => a.min(b).into(),
            (Value::Struct(a), Value::Struct(b)) => a.min(b).into(),
            _ => panic!("Invalid Min"),
        }
    }
}

impl Max for Value {
    fn max(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => a.clone().max(b.clone()).into(),
            (Value::Struct(a), Value::Struct(b)) => a.clone().max(b.clone()).into(),
            _ => panic!("Invalid Max {:#?}, {:#?}", self, rhs),
        }
    }
}

impl Sin for Value {
    fn sin(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().sin().into(),
            _ => panic!("Invalid Sin {:#?}", self),
        }
    }
}

impl Cos for Value {
    fn cos(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().cos().into(),
            _ => panic!("Invalid Cos {:#?}", self),
        }
    }
}

impl Tan for Value {
    fn tan(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().tan().into(),
            _ => panic!("Invalid Tan {:#?}", self),
        }
    }
}

impl Asin for Value {
    fn asin(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().asin().into(),
            _ => panic!("Invalid Asin {:#?}", self),
        }
    }
}

impl Acos for Value {
    fn acos(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().acos().into(),
            _ => panic!("Invalid Acos {:#?}", self),
        }
    }
}

impl Atan for Value {
    fn atan(self) -> Self {
        match &self {
            Value::Number(a) => a.clone().atan().into(),
            _ => panic!("Invalid Atan {:#?}", self),
        }
    }
}

impl Atan2 for Value {
    fn atan2(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => a.clone().atan2(b.clone()).into(),
            _ => panic!("Invalid Atan2 {:#?}, {:#?}", self, rhs),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Number(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Number(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Number(value.into())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Number(value.into())
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Number(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Number(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value.into())
    }
}

pub fn vector2<T>([x, y]: [T; 2]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(VECTOR2))
            .set(X.into(), x.into())
            .set(Y.into(), y.into()),
    )
}

pub fn vector3<T>([x, y, z]: [T; 3]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(VECTOR3))
            .set(X.into(), x.into())
            .set(Y.into(), y.into())
            .set(Z.into(), z.into()),
    )
}

pub fn vector4<T>([x, y, z, w]: [T; 4]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(VECTOR4))
            .set(X.into(), x.into())
            .set(Y.into(), y.into())
            .set(Z.into(), z.into())
            .set(W.into(), w.into()),
    )
}

pub fn matrix2<T>([x, y]: [[T; 2]; 2]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(MATRIX2))
            .set(X_AXIS_2.into(), vector2(x))
            .set(Y_AXIS_2.into(), vector2(y)),
    )
}

pub fn matrix3<T>([x, y, z]: [[T; 3]; 3]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(MATRIX3))
            .set(X_AXIS_3.into(), vector3(x))
            .set(Y_AXIS_3.into(), vector3(y))
            .set(Z_AXIS_3.into(), vector3(z)),
    )
}

pub fn matrix4<T>([x, y, z, w]: [[T; 4]; 4]) -> Value
where
    T: Clone,
    Value: From<T>,
{
    Value::Struct(
        Struct::new(StructIdentifier(MATRIX4))
            .set(X_AXIS_4.into(), vector4(x.clone()))
            .set(Y_AXIS_4.into(), vector4(y.clone()))
            .set(Z_AXIS_4.into(), vector4(z.clone()))
            .set(W_AXIS_4.into(), vector4(w.clone())),
    )
}

impl From<[f32; 2]> for Value {
    fn from([x, y]: [f32; 2]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR2))
                .set(X.into(), x.into())
                .set(Y.into(), y.into()),
        )
    }
}

impl From<[f64; 2]> for Value {
    fn from([x, y]: [f64; 2]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR2))
                .set(X.into(), x.into())
                .set(Y.into(), y.into()),
        )
    }
}

impl From<[f32; 3]> for Value {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR3))
                .set(X.into(), x.into())
                .set(Y.into(), y.into())
                .set(Z.into(), z.into()),
        )
    }
}

impl From<[f64; 3]> for Value {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR3))
                .set(X.into(), x.into())
                .set(Y.into(), y.into())
                .set(Z.into(), z.into()),
        )
    }
}

impl From<[f32; 4]> for Value {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR4))
                .set(X.into(), x.into())
                .set(Y.into(), y.into())
                .set(Z.into(), z.into())
                .set(W.into(), w.into()),
        )
    }
}

impl From<[f64; 4]> for Value {
    fn from([x, y, z, w]: [f64; 4]) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR4))
                .set(X.into(), x.into())
                .set(Y.into(), y.into())
                .set(Z.into(), z.into())
                .set(W.into(), w.into()),
        )
    }
}

impl From<Vec2> for Value {
    fn from(Vec2 { x, y }: Vec2) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR2))
                .set(X.into(), x.into())
                .set(Y.into(), y.into()),
        )
    }
}

impl From<Vec3> for Value {
    fn from(Vec3 { x, y, z }: Vec3) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR3))
                .set(X.into(), x.into())
                .set(Y.into(), y.into())
                .set(Z.into(), z.into()),
        )
    }
}

impl From<Vec4> for Value {
    fn from(value: Vec4) -> Self {
        Value::Struct(
            Struct::new(StructIdentifier(VECTOR4))
                .set(X.into(), value.x.into())
                .set(Y.into(), value.y.into())
                .set(Z.into(), value.z.into())
                .set(W.into(), value.w.into()),
        )
    }
}

impl From<Struct> for Value {
    fn from(value: Struct) -> Self {
        Value::Struct(value)
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl From<elysian_core::value::Value> for Value {
    fn from(value: elysian_core::value::Value) -> Self {
        match value {
            elysian_core::value::Value::Number(n) => Value::Number(n),
            elysian_core::value::Value::Vector2(v) => vector2(v),
            elysian_core::value::Value::Vector3(v) => vector3(v),
            elysian_core::value::Value::Vector4(v) => vector4(v),
            elysian_core::value::Value::Matrix2(m) => matrix2(m),
            elysian_core::value::Value::Matrix3(m) => matrix3(m),
            elysian_core::value::Value::Matrix4(m) => matrix4(m),
        }
    }
}

impl From<Box<elysian_core::value::Value>> for Box<Value> {
    fn from(value: Box<elysian_core::value::Value>) -> Self {
        Box::new(Value::from(*value))
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        let Value::Boolean(b) = value else {
        panic!("Value is not a Boolean")
    };

        b
    }
}

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for u16 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for u32 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for i8 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for i16 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for i32 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::UInt(n)) = value else {
            panic!("Value {value:#?} is not a UInt")
        };

        n as Self
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::Float(n)) = value else {
            panic!("Value {value:#?} is not a Float")
        };

        n as f32
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::Float(n)) = value else {
            panic!("Value is not a Float")
        };

        n
    }
}

impl From<Value> for [f32; 2] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [s.get(&X.into()).into(), s.get(&Y.into()).into()]
    }
}

impl From<Value> for [f64; 2] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [s.get(&X.into()).into(), s.get(&Y.into()).into()]
    }
}

impl From<Value> for [f32; 3] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
        ]
    }
}

impl From<Value> for [f64; 3] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
        ]
    }
}

impl From<Value> for [f32; 4] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
            s.get(&W.into()).into(),
        ]
    }
}

impl From<Value> for [f64; 4] {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        [
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
            s.get(&W.into()).into(),
        ]
    }
}

impl From<Value> for Vec2 {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        Vec2::new(s.get(&X.into()).into(), s.get(&Y.into()).into())
    }
}

impl From<Value> for Vec3 {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
            panic!("Value is not a Struct")
        };

        Vec3::new(
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
        )
    }
}

impl From<Value> for Vec4 {
    fn from(value: Value) -> Self {
        let Value::Struct(s) = value else {
        panic!("Value is not a Struct")
    };

        Vec4::new(
            s.get(&X.into()).into(),
            s.get(&Y.into()).into(),
            s.get(&Z.into()).into(),
            s.get(&W.into()).into(),
        )
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
