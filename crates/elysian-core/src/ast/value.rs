use rust_gpu_bridge::glam::{Vec2, Vec3, Vec4, DVec4, DVec3, DVec2};

use crate::ir::ast::TypeSpec;
use std::fmt::Debug;

#[non_exhaustive]
pub enum Value<T: TypeSpec> {
    Number(T::NUMBER),
    Vector2(T::VECTOR2),
    Vector3(T::VECTOR3),
    Vector4(T::VECTOR4),
}

impl<T> Debug for Value<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Vector2(arg0) => f.debug_tuple("Vector2").field(arg0).finish(),
            Self::Vector3(arg0) => f.debug_tuple("Vector3").field(arg0).finish(),
            Self::Vector4(arg0) => f.debug_tuple("Vector4").field(arg0).finish(),
        }
    }
}

impl<T> Clone for Value<T>
where
    T: TypeSpec,
    T::NUMBER: Clone,
    T::VECTOR2: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Vector2(arg0) => Self::Vector2(arg0.clone()),
            Self::Vector3(arg0) => Self::Vector3(arg0.clone()),
            Self::Vector4(arg0) => Self::Vector4(arg0.clone()),
        }
    }
}

impl<T> Copy for Value<T>
where
    T: TypeSpec,
    T::NUMBER: Copy,
    T::VECTOR2: Copy,
    T::VECTOR3: Copy,
    T::VECTOR4: Copy,
{
}

impl<T> std::hash::Hash for Value<T>
where
    T: TypeSpec,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait IntoValue<T>
where
    T: TypeSpec,
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
