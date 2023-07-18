use rust_gpu_bridge::glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};

use crate::ir::ast::{Struct, Value};

use super::{TypeSpec, VectorSpace};

pub trait IntoValue<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N> + ?Sized,
{
    fn value(self) -> Value<T, N>;
}

impl<T, const N: usize> IntoValue<T, N> for Value<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        self
    }
}

impl<T, const N: usize> IntoValue<T, N> for bool
where
    T: TypeSpec + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Boolean(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for i32
where
    T: TypeSpec<NUMBER = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Number(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for f32
where
    T: TypeSpec<NUMBER = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Number(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for f64
where
    T: TypeSpec<NUMBER = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Number(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f32; 2]
where
    T: TypeSpec<VECTOR2 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector2(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f32; 3]
where
    T: TypeSpec<VECTOR3 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector3(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f32; 4]
where
    T: TypeSpec<VECTOR4 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector4(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f64; 2]
where
    T: TypeSpec<VECTOR2 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector2(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f64; 3]
where
    T: TypeSpec<VECTOR3 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector3(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for [f64; 4]
where
    T: TypeSpec<VECTOR4 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector4(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for Vec2
where
    T: TypeSpec<VECTOR2 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector2(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for Vec3
where
    T: TypeSpec<VECTOR3 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector3(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for Vec4
where
    T: TypeSpec<VECTOR4 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector4(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for DVec2
where
    T: TypeSpec<VECTOR2 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector2(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for DVec3
where
    T: TypeSpec<VECTOR3 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector3(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for DVec4
where
    T: TypeSpec<VECTOR4 = Self> + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Vector4(self)
    }
}

impl<T, const N: usize> IntoValue<T, N> for Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn value(self) -> Value<T, N> {
        Value::Struct(self)
    }
}
