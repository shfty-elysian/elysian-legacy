use rust_gpu_bridge::glam::{Vec2, Vec3, Vec4};

use crate::ir::ast::Number;
use std::fmt::Debug;

#[non_exhaustive]
pub enum Value {
    Number(Number),
    Vector2(Number, Number),
    Vector3(Number, Number, Number),
    Vector4(Number, Number, Number, Number),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(value.into())
    }
}

impl From<Vec2> for Value {
    fn from(value: Vec2) -> Self {
        Value::Vector2(value.x.into(), value.y.into())
    }
}

impl From<Vec3> for Value {
    fn from(value: Vec3) -> Self {
        Value::Vector3(value.x.into(), value.y.into(), value.z.into())
    }
}

impl From<Vec4> for Value {
    fn from(value: Vec4) -> Self {
        Value::Vector4(
            value.x.into(),
            value.y.into(),
            value.z.into(),
            value.w.into(),
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
        match value {
            Value::Number(n) => n,
            _ => panic!("Value is not a Number"),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Vector2(arg0, arg1) => Self::Vector2(arg0.clone(), arg1.clone()),
            Self::Vector3(arg0, arg1, arg2) => {
                Self::Vector3(arg0.clone(), arg1.clone(), arg2.clone())
            }
            Self::Vector4(arg0, arg1, arg2, arg3) => {
                Self::Vector4(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone())
            }
        }
    }
}

impl Copy for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
