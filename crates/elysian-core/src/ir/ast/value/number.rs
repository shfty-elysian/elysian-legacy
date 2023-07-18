use std::ops::{Add, Div, Mul, Neg, Sub};

use rust_gpu_bridge::{Abs, Max, MaxBound, Min, Mix, One, Sign, Two, Zero};

use super::TypeSpec;

pub trait Number<T>:
    'static
    + Copy
    + Add<Output = T::NUMBER>
    + Sub<Output = T::NUMBER>
    + Mul<Output = T::NUMBER>
    + Div<Output = T::NUMBER>
    + Add<T::VECTOR2, Output = T::VECTOR2>
    + Sub<T::VECTOR2, Output = T::VECTOR2>
    + Mul<T::VECTOR2, Output = T::VECTOR2>
    + Div<T::VECTOR2, Output = T::VECTOR2>
    + MaxBound
    + Min
    + Max
    + Mix<T = T::NUMBER>
    + Neg<Output = T::NUMBER>
    + Abs
    + Sign
    + Zero
    + One
    + Two
    + PartialOrd
where
    T: TypeSpec + ?Sized,
{
}

impl<T, U> Number<U> for T
where
    U: TypeSpec,
    T: 'static
        + Copy
        + Add<Output = U::NUMBER>
        + Sub<Output = U::NUMBER>
        + Mul<Output = U::NUMBER>
        + Div<Output = U::NUMBER>
        + Add<U::VECTOR2, Output = U::VECTOR2>
        + Sub<U::VECTOR2, Output = U::VECTOR2>
        + Mul<U::VECTOR2, Output = U::VECTOR2>
        + Div<U::VECTOR2, Output = U::VECTOR2>
        + MaxBound
        + Min
        + Max
        + Mix<T = U::NUMBER>
        + Neg<Output = U::NUMBER>
        + Abs
        + Sign
        + Zero
        + One
        + Two
        + PartialOrd,
{
}
