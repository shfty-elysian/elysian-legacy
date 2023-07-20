mod structure;

use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec4},
    Abs, Dot, Length, Max, Min, Mix, Normalize, Sign,
};
pub use structure::*;

use super::Expr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Float(f32),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(n) => write!(f, "{n:}"),
        }
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value)
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        match value {
            Number::Float(n) => n,
        }
    }
}

impl Number {
    pub fn literal(self) -> Expr {
        Expr::Literal(Value::Number(self))
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
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
            panic!("Value is not a Number")
        };

        n
    }
}

/// Concrete value
#[non_exhaustive]
pub enum Value {
    Boolean(bool),
    Number(Number),
    Vector2(Number, Number),
    Vector3(Number, Number, Number),
    Vector4(Number, Number, Number, Number),
    Struct(Struct),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{b:}"),
            Value::Number(n) => write!(f, "{n:}"),
            Value::Vector2(x, y) => write!(f, "({x:}, {y:})"),
            Value::Vector3(x, y, z) => write!(f, "({x:}, {y:}, {z:})"),
            Value::Vector4(x, y, z, w) => write!(f, "({x:}, {y:}, {z:}, {w:})"),
            Value::Struct(s) => write!(f, "{s:}"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => a.partial_cmp(b),
            _ => panic!("Invalid PartialOrd"),
        }
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
        let Value::Number(Number::Float(n)) = value else {
            panic!("Value is not a Float Number")
        };

        n
    }
}

impl From<Value> for Vec2 {
    fn from(value: Value) -> Self {
        let Value::Vector2(Number::Float(x), Number::Float(y)) = value else {
            panic!("Value is not a Float Vector2")
        };

        Vec2::new(x, y)
    }
}

impl From<Value> for Vec3 {
    fn from(value: Value) -> Self {
        let Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) = value else {
            panic!("Value is not a Vector3")
        };

        Vec3::new(x, y, z)
    }
}

impl From<Value> for Vec4 {
    fn from(value: Value) -> Self {
        let Value::Vector4(Number::Float(x), Number::Float(y), Number::Float(z), Number::Float(w)) = value else {
            panic!("Value is not a Vector4")
        };

        Vec4::new(x, y, z, w)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(Number::Float(value))
    }
}

impl From<Vec2> for Value {
    fn from(value: Vec2) -> Self {
        Value::Vector2(Number::Float(value.x), Number::Float(value.y))
    }
}

impl From<Vec3> for Value {
    fn from(value: Vec3) -> Self {
        Value::Vector3(
            Number::Float(value.x),
            Number::Float(value.y),
            Number::Float(value.z),
        )
    }
}

impl From<Vec4> for Value {
    fn from(value: Vec4) -> Self {
        Value::Vector4(
            Number::Float(value.x),
            Number::Float(value.y),
            Number::Float(value.z),
            Number::Float(value.w),
        )
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Vector2(arg0, arg1) => f.debug_tuple("Vector2").field(arg0).field(arg1).finish(),
            Self::Vector3(arg0, arg1, arg2) => f
                .debug_tuple("Vector3")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Vector4(arg0, arg1, arg2, arg3) => f
                .debug_tuple("Vector4")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .field(arg3)
                .finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(arg0).finish(),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Vector2(arg0, arg1) => Self::Vector2(arg0.clone(), arg1.clone()),
            Self::Vector3(arg0, arg1, arg2) => {
                Self::Vector3(arg0.clone(), arg1.clone(), arg2.clone())
            }
            Self::Vector4(arg0, arg1, arg2, arg3) => {
                Self::Vector4(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone())
            }
            Self::Struct(arg0) => Self::Struct(arg0.clone()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Vector2(l0, l1), Self::Vector2(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Vector3(l0, l1, l2), Self::Vector3(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            (Self::Vector4(l0, l1, l2, l3), Self::Vector4(r0, r1, r2, r3)) => {
                l0 == r0 && l1 == r1 && l2 == r2 && l3 == r3
            }
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            _ => false,
        }
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
            crate::ast::value::Value::Number(Number::Float(n)) => Value::Number(Number::Float(n)),
            crate::ast::value::Value::Vector2(Number::Float(x), Number::Float(y)) => {
                Value::Vector2(Number::Float(x), Number::Float(y))
            }
            crate::ast::value::Value::Vector3(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
            ) => Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
            crate::ast::value::Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ),
        }
    }
}

impl From<Box<crate::ast::value::Value>> for Box<Value> {
    fn from(value: Box<crate::ast::value::Value>) -> Self {
        Box::new(Value::from(*value))
    }
}

impl Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => (a + b).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector2(Number::Float(x), Number::Float(y)),
            ) => (Vec2::new(x, y) + n).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
            ) => (n + Vec3::new(x, y, z)).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
            ) => (n + Vec4::new(x, y, z, w)).into(),
            (
                Value::Vector2(Number::Float(x), Number::Float(y)),
                Value::Number(Number::Float(n)),
            ) => (Vec2::new(x, y) + n).into(),
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => (Vec2::new(lx, ly) + Vec2::new(rx, ry)).into(),
            (
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
                Value::Number(Number::Float(n)),
            ) => (Vec3::new(x, y, z) + n).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => (Vec3::new(lx, ly, lz) + Vec3::new(rx, ry, rz)).into(),
            (
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
                Value::Number(Number::Float(n)),
            ) => (Vec4::new(x, y, z, w) + n).into(),
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Self;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => (a - b).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector2(Number::Float(x), Number::Float(y)),
            ) => (Vec2::new(x, y) - n).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
            ) => (n - Vec3::new(x, y, z)).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
            ) => (n - Vec4::new(x, y, z, w)).into(),
            (
                Value::Vector2(Number::Float(x), Number::Float(y)),
                Value::Number(Number::Float(n)),
            ) => (Vec2::new(x, y) - n).into(),
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => (Vec2::new(lx, ly) - Vec2::new(rx, ry)).into(),
            (
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
                Value::Number(Number::Float(n)),
            ) => (Vec3::new(x, y, z) - n).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => (Vec3::new(lx, ly, lz) - Vec3::new(rx, ry, rz)).into(),
            (
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
                Value::Number(Number::Float(n)),
            ) => (Vec4::new(x, y, z, w) - n).into(),
            _ => panic!("Invalid Sub"),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => (a * b).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector2(Number::Float(x), Number::Float(y)),
            ) => (Vec2::new(x, y) * n).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
            ) => (n * Vec3::new(x, y, z)).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
            ) => (n * Vec4::new(x, y, z, w)).into(),
            (
                Value::Vector2(Number::Float(x), Number::Float(y)),
                Value::Number(Number::Float(n)),
            ) => (Vec2::new(x, y) * n).into(),
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => (Vec2::new(lx, ly) * Vec2::new(rx, ry)).into(),
            (
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
                Value::Number(Number::Float(n)),
            ) => (Vec3::new(x, y, z) * n).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => (Vec3::new(lx, ly, lz) * Vec3::new(rx, ry, rz)).into(),
            (
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
                Value::Number(Number::Float(n)),
            ) => (Vec4::new(x, y, z, w) * n).into(),
            _ => panic!("Invalid Mul"),
        }
    }
}

impl Div<Value> for Value {
    type Output = Self;

    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => (a / b).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector2(Number::Float(x), Number::Float(y)),
            ) => (Vec2::new(x, y) / n).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
            ) => (n / Vec3::new(x, y, z)).into(),
            (
                Value::Number(Number::Float(n)),
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
            ) => (n / Vec4::new(x, y, z, w)).into(),
            (
                Value::Vector2(Number::Float(x), Number::Float(y)),
                Value::Number(Number::Float(n)),
            ) => (Vec2::new(x, y) / n).into(),
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => (Vec2::new(lx, ly) / Vec2::new(rx, ry)).into(),
            (
                Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)),
                Value::Number(Number::Float(n)),
            ) => (Vec3::new(x, y, z) / n).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => (Vec3::new(lx, ly, lz) / Vec3::new(rx, ry, rz)).into(),
            (
                Value::Vector4(
                    Number::Float(x),
                    Number::Float(y),
                    Number::Float(z),
                    Number::Float(w),
                ),
                Value::Number(Number::Float(n)),
            ) => (Vec4::new(x, y, z, w) / n).into(),
            _ => panic!("Invalid Mul"),
        }
    }
}

impl Mix for Value {
    type T = Value;

    fn mix(self, to: Self, t: Self::T) -> Self {
        let Value::Number(Number::Float(t)) = t else {
            panic!("T is not a Number");
        };

        match (self, to) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => {
                a.mix(b, t).into()
            }
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => Vec2::new(lx, ly).mix(Vec2::new(rx, ry), t).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => Vec3::new(lx, ly, lz).mix(Vec3::new(rx, ry, rz), t).into(),
            (
                Value::Vector4(
                    Number::Float(lx),
                    Number::Float(ly),
                    Number::Float(lz),
                    Number::Float(lw),
                ),
                Value::Vector4(
                    Number::Float(rx),
                    Number::Float(ry),
                    Number::Float(rz),
                    Number::Float(rw),
                ),
            ) => Vec4::new(lx, ly, lz, lw)
                .mix(Vec4::new(rx, ry, rz, rw), t)
                .into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(Number::Float(n)) => (-n).into(),
            Value::Vector2(Number::Float(x), Number::Float(y)) => (-Vec2::new(x, y)).into(),
            Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) => {
                (-Vec3::new(x, y, z)).into()
            }
            Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => (-Vec4::new(x, y, z, w)).into(),
            _ => panic!("Invalid Neg"),
        }
    }
}

impl Abs for Value {
    fn abs(self) -> Self {
        match self {
            Value::Number(Number::Float(n)) => n.abs().into(),
            Value::Vector2(Number::Float(x), Number::Float(y)) => Vec2::new(x, y).abs().into(),
            Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) => {
                Vec3::new(x, y, z).abs().into()
            }
            Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => Vec4::new(x, y, z, w).abs().into(),
            _ => panic!("Invalid Abs"),
        }
    }
}

impl Sign for Value {
    fn sign(self) -> Self {
        match self {
            Value::Number(Number::Float(n)) => n.sign().into(),
            Value::Vector2(Number::Float(x), Number::Float(y)) => Vec2::new(x, y).sign().into(),
            Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) => {
                Vec3::new(x, y, z).sign().into()
            }
            Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => Vec4::new(x, y, z, w).sign().into(),
            _ => panic!("Invalid Sign"),
        }
    }
}

impl Length for Value {
    type Output = Self;

    fn length(self) -> Self::Output {
        match self {
            Value::Number(Number::Float(n)) => n.abs().into(),
            Value::Vector2(Number::Float(x), Number::Float(y)) => Vec2::new(x, y).length().into(),
            Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) => {
                Vec3::new(x, y, z).length().into()
            }
            Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => Vec4::new(x, y, z, w).length().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Normalize for Value {
    fn normalize(self) -> Self {
        match self {
            Value::Number(Number::Float(n)) => n.sign().into(),
            Value::Vector2(Number::Float(x), Number::Float(y)) => {
                Vec2::new(x, y).normalize().into()
            }
            Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) => {
                Vec3::new(x, y, z).normalize().into()
            }
            Value::Vector4(
                Number::Float(x),
                Number::Float(y),
                Number::Float(z),
                Number::Float(w),
            ) => Vec4::new(x, y, z, w).normalize().into(),
            _ => panic!("Invalid Normalize"),
        }
    }
}

impl Dot for Value {
    type Output = Self;

    fn dot(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => (a * b).into(),
            (
                Value::Vector2(Number::Float(lx), Number::Float(ly)),
                Value::Vector2(Number::Float(rx), Number::Float(ry)),
            ) => Vec2::new(lx, ly).dot(Vec2::new(rx, ry)).into(),
            (
                Value::Vector3(Number::Float(lx), Number::Float(ly), Number::Float(lz)),
                Value::Vector3(Number::Float(rx), Number::Float(ry), Number::Float(rz)),
            ) => Vec3::new(lx, ly, lz).dot(Vec3::new(rx, ry, rz)).into(),
            (
                Value::Vector4(
                    Number::Float(lx),
                    Number::Float(ly),
                    Number::Float(lz),
                    Number::Float(lw),
                ),
                Value::Vector4(
                    Number::Float(rx),
                    Number::Float(ry),
                    Number::Float(rz),
                    Number::Float(rw),
                ),
            ) => Vec4::new(lx, ly, lz, lw)
                .dot(Vec4::new(rx, ry, rz, rw))
                .into(),
            _ => panic!("Invalid Dot"),
        }
    }
}

impl Min for Value {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => a.min(b).into(),
            _ => panic!("Invalid Min"),
        }
    }
}

impl Max for Value {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(Number::Float(a)), Value::Number(Number::Float(b))) => a.max(b).into(),
            _ => panic!("Invalid Min"),
        }
    }
}
