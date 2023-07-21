use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec4},
    Abs, Dot, Length, Max, Min, Mix, Normalize, Sign,
};

use super::{Float, Matrix, Number, Struct, Vector};

/// Concrete value
#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(Number),
    Vector(Vector),
    Matrix(Matrix),
    Struct(Struct),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{b:}"),
            Value::Number(n) => write!(f, "{n:}"),
            Value::Vector(v) => write!(f, "{v:}"),
            Value::Matrix(m) => write!(f, "{m:}"),
            Value::Struct(s) => write!(f, "{s:}"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Vector(l0), Self::Vector(r0)) => l0 == r0,
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
            _ => panic!("Invalid PartialOrd"),
        }
    }
}

impl Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a + b).into(),
            (Value::Number(a), Value::Vector(b)) => (a + b).into(),
            (Value::Vector(a), Value::Number(b)) => (a + b).into(),
            (Value::Vector(a), Value::Vector(b)) => (a + b).into(),
            (Value::Matrix(a), Value::Matrix(b)) => (a + b).into(),
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Self;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a - b).into(),
            (Value::Number(a), Value::Vector(b)) => (a - b).into(),
            (Value::Vector(a), Value::Number(b)) => (a - b).into(),
            (Value::Vector(a), Value::Vector(b)) => (a - b).into(),
            (Value::Matrix(a), Value::Matrix(b)) => (a - b).into(),
            _ => panic!("Invalid Sub"),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a * b).into(),
            (Value::Number(a), Value::Vector(b)) => (a * b).into(),
            (Value::Vector(a), Value::Number(b)) => (a * b).into(),
            (Value::Vector(a), Value::Vector(b)) => (a * b).into(),
            (Value::Matrix(a), Value::Matrix(b)) => (a * b).into(),
            (Value::Matrix(a), Value::Vector(b)) => (a * b).into(),
            (Value::Matrix(a), Value::Number(b)) => (a * b).into(),
            t => panic!("Invalid Mul {t:#?}"),
        }
    }
}

impl Div<Value> for Value {
    type Output = Self;

    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a / b).into(),
            (Value::Number(a), Value::Vector(b)) => (a / b).into(),
            (Value::Vector(a), Value::Number(b)) => (a / b).into(),
            (Value::Vector(a), Value::Vector(b)) => (a / b).into(),
            _ => panic!("Invalid Div"),
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
            (Value::Vector(a), Value::Vector(b)) => a.mix(b, t).into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => (-n).into(),
            Value::Vector(v) => (-v).into(),
            _ => panic!("Invalid Neg"),
        }
    }
}

impl Abs for Value {
    fn abs(self) -> Self {
        match self {
            Value::Number(n) => n.abs().into(),
            Value::Vector(v) => v.abs().into(),
            _ => panic!("Invalid Abs"),
        }
    }
}

impl Sign for Value {
    fn sign(self) -> Self {
        match self {
            Value::Number(n) => n.sign().into(),
            Value::Vector(v) => v.sign().into(),
            _ => panic!("Invalid Sign"),
        }
    }
}

impl Length for Value {
    type Output = Self;

    fn length(self) -> Self::Output {
        match self {
            Value::Number(n) => n.abs().into(),
            Value::Vector(v) => v.length().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Normalize for Value {
    fn normalize(self) -> Self {
        match self {
            Value::Number(n) => n.sign().into(),
            Value::Vector(v) => v.normalize().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Dot for Value {
    type Output = Self;

    fn dot(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => (a * b).into(),
            (Value::Vector(a), Value::Vector(b)) => a.dot(b).into(),
            _ => panic!("Invalid Dot"),
        }
    }
}

impl Min for Value {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => a.min(b).into(),
            (Value::Vector(a), Value::Vector(b)) => a.min(b).into(),
            _ => panic!("Invalid Min"),
        }
    }
}

impl Max for Value {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => a.max(b).into(),
            (Value::Vector(a), Value::Vector(b)) => a.max(b).into(),
            _ => panic!("Invalid Min"),
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

impl<T> From<[T; 2]> for Value
where
    T: Clone,
    Number: From<T>,
{
    fn from(value: [T; 2]) -> Self {
        Value::Vector(Vector::Vector2(
            value[0].clone().into(),
            value[1].clone().into(),
        ))
    }
}

impl<T> From<[T; 3]> for Value
where
    T: Clone,
    Number: From<T>,
{
    fn from(value: [T; 3]) -> Self {
        Value::Vector(Vector::Vector3(
            value[0].clone().into(),
            value[1].clone().into(),
            value[2].clone().into(),
        ))
    }
}

impl<T> From<[T; 4]> for Value
where
    T: Clone,
    Number: From<T>,
{
    fn from(value: [T; 4]) -> Self {
        Value::Vector(Vector::Vector4(
            value[0].clone().into(),
            value[1].clone().into(),
            value[2].clone().into(),
            value[3].clone().into(),
        ))
    }
}

impl From<Vec2> for Value {
    fn from(value: Vec2) -> Self {
        Value::Vector(Vector::Vector2(
            Number::Float(value.x.into()),
            Number::Float(value.y.into()),
        ))
    }
}

impl From<Vec3> for Value {
    fn from(value: Vec3) -> Self {
        Value::Vector(Vector::Vector3(
            Number::Float(value.x.into()),
            Number::Float(value.y.into()),
            Number::Float(value.z.into()),
        ))
    }
}

impl From<Vec4> for Value {
    fn from(value: Vec4) -> Self {
        Value::Vector(Vector::Vector4(
            Number::Float(value.x.into()),
            Number::Float(value.y.into()),
            Number::Float(value.z.into()),
            Number::Float(value.w.into()),
        ))
    }
}

impl From<Vector> for Value {
    fn from(value: Vector) -> Self {
        Value::Vector(value)
    }
}

impl From<Matrix> for Value {
    fn from(value: Matrix) -> Self {
        Value::Matrix(value)
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl From<crate::ast::value::Value> for Value {
    fn from(value: crate::ast::value::Value) -> Self {
        match value {
            crate::ast::value::Value::Number(n) => Value::Number(n),
            crate::ast::value::Value::Vector2(x, y) => Value::Vector(Vector::Vector2(x, y)),
            crate::ast::value::Value::Vector3(x, y, z) => Value::Vector(Vector::Vector3(x, y, z)),
            crate::ast::value::Value::Vector4(x, y, z, w) => {
                Value::Vector(Vector::Vector4(x, y, z, w))
            }
            crate::ast::value::Value::Matrix2(x, y) => Value::Matrix(Matrix::Matrix2(x, y)),
            crate::ast::value::Value::Matrix3(x, y, z) => Value::Matrix(Matrix::Matrix3(x, y, z)),
            crate::ast::value::Value::Matrix4(x, y, z, w) => {
                Value::Matrix(Matrix::Matrix4(x, y, z, w))
            }
        }
    }
}

impl From<Box<crate::ast::value::Value>> for Box<Value> {
    fn from(value: Box<crate::ast::value::Value>) -> Self {
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

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::Float(Float::F32(n))) = value else {
        panic!("Value is not a f32")
    };

        n
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        let Value::Number(Number::Float(Float::F64(n))) = value else {
        panic!("Value is not a f64")
    };

        n
    }
}

impl From<Value> for Vec2 {
    fn from(value: Value) -> Self {
        let Value::Vector(Vector::Vector2(Number::Float(Float::F32(x)), Number::Float(Float::F32(y)))) = value else {
        panic!("Value is not a Float Vector2")
    };

        Vec2::new(x, y)
    }
}

impl From<Value> for Vec3 {
    fn from(value: Value) -> Self {
        let Value::Vector(Vector::Vector3(Number::Float(Float::F32(x)), Number::Float(Float::F32(y)), Number::Float(Float::F32(z)))) = value else {
        panic!("Value is not a Vector3")
    };

        Vec3::new(x, y, z)
    }
}

impl From<Value> for Vec4 {
    fn from(value: Value) -> Self {
        let Value::Vector(Vector::Vector4(Number::Float(Float::F32(x)), Number::Float(Float::F32(y)), Number::Float(Float::F32(z)), Number::Float(Float::F32(w)))) = value else {
        panic!("Value is not a Vector4")
    };

        Vec4::new(x, y, z, w)
    }
}
