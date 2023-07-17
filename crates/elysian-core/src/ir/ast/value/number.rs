use std::ops::{Add, Div, Mul, Neg, Sub};

use rust_gpu_bridge::{Abs, Max, MaxBound, Min, Mix, One, Sign, Two, Zero};

use crate::ir::ast::IntoValue;

pub trait Number<N, V>:
    'static
    + Copy
    + Add<Output = N>
    + Sub<Output = N>
    + Mul<Output = N>
    + Div<Output = N>
    + Add<V, Output = V>
    + Sub<V, Output = V>
    + Mul<V, Output = V>
    + Div<V, Output = V>
    + MaxBound
    + Min
    + Max
    + Mix<T = N>
    + Neg<Output = N>
    + Abs
    + Sign
    + Zero
    + One
    + Two
    + IntoValue<N, V>
    + PartialOrd
{
}

impl<T, N, V> Number<N, V> for T where
    T: 'static
        + Copy
        + Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Add<V, Output = V>
        + Sub<V, Output = V>
        + Mul<V, Output = V>
        + Div<V, Output = V>
        + MaxBound
        + Min
        + Max
        + Mix<T = N>
        + Neg<Output = N>
        + Abs
        + Sign
        + Zero
        + One
        + Two
        + IntoValue<N, V>
        + PartialOrd
{
}
