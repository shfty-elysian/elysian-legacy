use rust_gpu_bridge::glam::Mat4;

use crate::ir::ast::Number;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(Number),
    Vector2([Number; 2]),
    Vector3([Number; 3]),
    Vector4([Number; 4]),
    Matrix2([[Number; 2]; 2]),
    Matrix3([[Number; 3]; 3]),
    Matrix4([[Number; 4]; 4]),
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
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
        Value::Vector2([value[0].clone().into(), value[1].clone().into()])
    }
}

impl<T> From<[T; 3]> for Value
where
    T: Clone,
    Number: From<T>,
{
    fn from(value: [T; 3]) -> Self {
        Value::Vector3([
            value[0].clone().into(),
            value[1].clone().into(),
            value[2].clone().into(),
        ])
    }
}

impl<T> From<[T; 4]> for Value
where
    T: Clone,
    Number: From<T>,
{
    fn from(value: [T; 4]) -> Self {
        Value::Vector4([
            value[0].clone().into(),
            value[1].clone().into(),
            value[2].clone().into(),
            value[3].clone().into(),
        ])
    }
}

impl From<Mat4> for Value {
    fn from(value: Mat4) -> Self {
        Value::Matrix4([
            [
                value.x_axis.x.into(),
                value.x_axis.y.into(),
                value.x_axis.z.into(),
                value.x_axis.w.into(),
            ],
            [
                value.y_axis.x.into(),
                value.y_axis.y.into(),
                value.y_axis.z.into(),
                value.y_axis.w.into(),
            ],
            [
                value.z_axis.x.into(),
                value.z_axis.y.into(),
                value.z_axis.z.into(),
                value.z_axis.w.into(),
            ],
            [
                value.w_axis.x.into(),
                value.w_axis.y.into(),
                value.w_axis.z.into(),
                value.w_axis.w.into(),
            ],
        ])
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
