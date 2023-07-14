use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::ir::ast::{Struct, Value};

pub trait IntoValue<N, V> {
    fn value(self) -> Value<N, V>;
}

impl<N, V> IntoValue<N, V> for Value<N, V> {
    fn value(self) -> Value<N, V> {
        self
    }
}

impl<V> IntoValue<f32, V> for bool {
    fn value(self) -> Value<f32, V> {
        Value::Boolean(self)
    }
}

impl<V> IntoValue<f32, V> for f32 {
    fn value(self) -> Value<f32, V> {
        Value::Number(self)
    }
}

impl<N> IntoValue<N, Self> for [f32; 2] {
    fn value(self) -> Value<N, Self> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, Self> for [f32; 3] {
    fn value(self) -> Value<N, Self> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, Vec2> for Vec2 {
    fn value(self) -> Value<N, Vec2> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, Vec3> for Vec3 {
    fn value(self) -> Value<N, Vec3> {
        Value::Vector(self)
    }
}

impl<N, V> IntoValue<N, V> for Struct<N, V> {
    fn value(self) -> Value<N, V> {
        Value::Struct(self)
    }
}
