use crate::ir::ast::IntoValue;
use rust_gpu_bridge::{Abs, Dot, Length, Mix, Normalize};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Vector<N, V>:
    'static
    + Copy
    + Default
    + Add<Output = V>
    + Sub<Output = V>
    + Mul<Output = V>
    + Div<Output = V>
    + Add<N, Output = V>
    + Sub<N, Output = V>
    + Mul<N, Output = V>
    + Div<N, Output = V>
    + Mix<T = N>
    + Length<Output = N>
    + Normalize
    + Neg<Output = V>
    + Abs
    + IntoValue<N, V>
    + Dot<Output = N>
{
}

impl<T, N, V> Vector<N, V> for T where
    T: 'static
        + Copy
        + Default
        + Add<Output = V>
        + Sub<Output = V>
        + Mul<Output = V>
        + Div<Output = V>
        + Add<N, Output = V>
        + Sub<N, Output = V>
        + Mul<N, Output = V>
        + Div<N, Output = V>
        + Mix<T = N>
        + Length<Output = N>
        + Normalize
        + Neg<Output = V>
        + Abs
        + IntoValue<N, V>
        + Dot<Output = N>
{
}
