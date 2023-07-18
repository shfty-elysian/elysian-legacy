use rust_gpu_bridge::{Abs, Dot, Length, Mix, Normalize};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::TypeSpec;

pub trait Vector2<T>:
    'static
    + Copy
    + Default
    + Add<Output = T::VECTOR2>
    + Sub<Output = T::VECTOR2>
    + Mul<Output = T::VECTOR2>
    + Div<Output = T::VECTOR2>
    + Add<T::NUMBER, Output = T::VECTOR2>
    + Sub<T::NUMBER, Output = T::VECTOR2>
    + Mul<T::NUMBER, Output = T::VECTOR2>
    + Div<T::NUMBER, Output = T::VECTOR2>
    + Mix<T = T::NUMBER>
    + Length<Output = T::NUMBER>
    + Normalize
    + Neg<Output = T::VECTOR2>
    + Abs
    + Dot<Output = T::NUMBER>
where
    T: TypeSpec + ?Sized,
{
}

impl<T, U> Vector2<U> for T
where
    U: TypeSpec,
    T: 'static
        + Copy
        + Default
        + Add<Output = U::VECTOR2>
        + Sub<Output = U::VECTOR2>
        + Mul<Output = U::VECTOR2>
        + Div<Output = U::VECTOR2>
        + Add<U::NUMBER, Output = U::VECTOR2>
        + Sub<U::NUMBER, Output = U::VECTOR2>
        + Mul<U::NUMBER, Output = U::VECTOR2>
        + Div<U::NUMBER, Output = U::VECTOR2>
        + Mix<T = U::NUMBER>
        + Length<Output = U::NUMBER>
        + Normalize
        + Neg<Output = U::VECTOR2>
        + Abs
        + Dot<Output = U::NUMBER>,
{
}
