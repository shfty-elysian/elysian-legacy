mod into_value;
mod number;
mod structure;
mod type_spec;
mod vector;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub use into_value::*;
pub use number::*;
pub use structure::*;
pub use type_spec::*;
pub use vector::*;

pub trait VectorSpace<const N: usize> {
    type VectorSpace: 'static + Debug + Clone + PartialEq;
}

impl<T> VectorSpace<1> for T
where
    T: TypeSpec + ?Sized,
{
    type VectorSpace = T::NUMBER;
}

impl<T> VectorSpace<2> for T
where
    T: TypeSpec + ?Sized,
{
    type VectorSpace = T::VECTOR2;
}

impl<T> VectorSpace<3> for T
where
    T: TypeSpec + ?Sized,
{
    type VectorSpace = T::VECTOR3;
}

impl<T> VectorSpace<4> for T
where
    T: TypeSpec + ?Sized,
{
    type VectorSpace = T::VECTOR4;
}

pub type VectorSpaceT<T, const N: usize> = <T as VectorSpace<N>>::VectorSpace;

/// Concrete value
#[non_exhaustive]
pub enum Value<T>
where
    T: TypeSpec + ?Sized,
{
    Boolean(bool),
    Number(T::NUMBER),
    Vector2(T::VECTOR2),
    Vector3(T::VECTOR3),
    Vector4(T::VECTOR4),
    Struct(Struct<T>),
}

impl<T> Debug for Value<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Vector2(arg0) => f.debug_tuple("Vector2").field(arg0).finish(),
            Self::Vector3(arg0) => f.debug_tuple("Vector3").field(arg0).finish(),
            Self::Vector4(arg0) => f.debug_tuple("Vector4").field(arg0).finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(arg0).finish(),
        }
    }
}

impl<T> Clone for Value<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        match self {
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Vector2(arg0) => Self::Vector2(arg0.clone()),
            Self::Vector3(arg0) => Self::Vector3(arg0.clone()),
            Self::Vector4(arg0) => Self::Vector4(arg0.clone()),
            Self::Struct(arg0) => Self::Struct(arg0.clone()),
        }
    }
}

impl<T> PartialEq for Value<T>
where
    T: TypeSpec,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Vector2(l0), Self::Vector2(r0)) => l0 == r0,
            (Self::Vector3(l0), Self::Vector3(r0)) => l0 == r0,
            (Self::Vector4(l0), Self::Vector4(r0)) => l0 == r0,
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<T> Hash for Value<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<T> From<crate::ast::value::Value<T>> for Value<T>
where
    T: TypeSpec,
{
    fn from(value: crate::ast::value::Value<T>) -> Self {
        match value {
            crate::ast::value::Value::Number(n) => Value::Number(n),
            crate::ast::value::Value::Vector2(v) => Value::Vector2(v),
            crate::ast::value::Value::Vector3(v) => Value::Vector3(v),
            crate::ast::value::Value::Vector4(v) => Value::Vector4(v),
        }
    }
}

impl<T> From<Box<crate::ast::value::Value<T>>> for Box<Value<T>>
where
    T: TypeSpec,
{
    fn from(value: Box<crate::ast::value::Value<T>>) -> Self {
        Box::new(Value::from(*value))
    }
}
