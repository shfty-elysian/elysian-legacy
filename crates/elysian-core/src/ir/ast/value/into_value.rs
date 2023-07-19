use rust_gpu_bridge::glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};

use crate::ir::ast::{Struct, Value};

use super::TypeSpec;

pub trait IntoValue<T>
where
    T: TypeSpec + ?Sized,
{
    fn value(self) -> Value<T>;
}

impl<T> IntoValue<T> for Value<T>
where
    T: TypeSpec,
{
    fn value(self) -> Value<T> {
        self
    }
}

impl<T> IntoValue<T> for bool
where
    T: TypeSpec,
{
    fn value(self) -> Value<T> {
        Value::Boolean(self)
    }
}

impl<T> IntoValue<T> for i32
where
    T: TypeSpec<NUMBER = Self>,
{
    fn value(self) -> Value<T> {
        Value::Number(self)
    }
}

impl<T> IntoValue<T> for f32
where
    T: TypeSpec<NUMBER = Self>,
{
    fn value(self) -> Value<T> {
        Value::Number(self)
    }
}

impl<T> IntoValue<T> for f64
where
    T: TypeSpec<NUMBER = Self>,
{
    fn value(self) -> Value<T> {
        Value::Number(self)
    }
}

impl<T> IntoValue<T> for [f32; 2]
where
    T: TypeSpec<VECTOR2 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector2(self)
    }
}

impl<T> IntoValue<T> for [f32; 3]
where
    T: TypeSpec<VECTOR3 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector3(self)
    }
}

impl<T> IntoValue<T> for [f32; 4]
where
    T: TypeSpec<VECTOR4 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector4(self)
    }
}

impl<T> IntoValue<T> for [f64; 2]
where
    T: TypeSpec<VECTOR2 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector2(self)
    }
}

impl<T> IntoValue<T> for [f64; 3]
where
    T: TypeSpec<VECTOR3 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector3(self)
    }
}

impl<T> IntoValue<T> for [f64; 4]
where
    T: TypeSpec<VECTOR4 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector4(self)
    }
}

impl<T> IntoValue<T> for Vec2
where
    T: TypeSpec<VECTOR2 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector2(self)
    }
}

impl<T> IntoValue<T> for Vec3
where
    T: TypeSpec<VECTOR3 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector3(self)
    }
}

impl<T> IntoValue<T> for Vec4
where
    T: TypeSpec<VECTOR4 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector4(self)
    }
}

impl<T> IntoValue<T> for DVec2
where
    T: TypeSpec<VECTOR2 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector2(self)
    }
}

impl<T> IntoValue<T> for DVec3
where
    T: TypeSpec<VECTOR3 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector3(self)
    }
}

impl<T> IntoValue<T> for DVec4
where
    T: TypeSpec<VECTOR4 = Self>,
{
    fn value(self) -> Value<T> {
        Value::Vector4(self)
    }
}

impl<T> IntoValue<T> for Struct<T>
where
    T: TypeSpec,
{
    fn value(self) -> Value<T> {
        Value::Struct(self)
    }
}
