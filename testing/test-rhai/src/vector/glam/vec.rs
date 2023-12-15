use crate::Vector;
use glam::{Vec2, Vec3, Vec4};
use rhai::{Array, Dynamic, EvalAltResult, FLOAT, INT};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_vec {
    ($ident:ident, $name:expr, $($new:tt)*) => {
        impl Vector for $ident {
            fn name() -> &'static str {
                $name
            }

            fn new() -> fn(Array) -> Result<Self, Box<EvalAltResult>> {
                $($new)*
            }

            fn index() -> fn(&mut Self, INT) -> FLOAT {
                |x, idx| {
                    if idx < 0 || idx >= Self::AXES.len() as INT {
                        Default::default()
                    }
                    else {
                        x[idx as usize] as FLOAT
                    }
                }
            }

            fn splat() -> fn(FLOAT) -> Self {
                |x| Self::splat(x as f32)
            }

            fn length() -> fn(Self) -> FLOAT {
                |x| Self::length(x) as FLOAT
            }

            fn normalize() -> fn(Self) -> Self {
                Self::normalize
            }

            fn abs() -> fn(Self) -> Self {
                Self::abs
            }

            fn sign() -> fn(Self) -> Self {
                Self::signum
            }

            fn min_vec() -> fn(Self, Self) -> Self {
                Self::min
            }

            fn max_vec() -> fn(Self, Self) -> Self {
                Self::max
            }

            fn add_vec() -> fn(Self, Self) -> Self {
                Self::add
            }

            fn add_float() -> fn(Self, FLOAT) -> Self {
                |a, b| Self::add(a, b as f32)
            }

            fn sub_vec() -> fn(Self, Self) -> Self {
                Self::sub
            }

            fn sub_float() -> fn(Self, FLOAT) -> Self {
                |a, b| Self::sub(a, b as f32)
            }

            fn mul_vec() -> fn(Self, Self) -> Self {
                Self::mul
            }

            fn mul_float() -> fn(Self, FLOAT) -> Self {
                |a, b| Self::mul(a, b as f32)
            }

            fn div_vec() -> fn(Self, Self) -> Self {
                Self::div
            }

            fn div_float() -> fn(Self, FLOAT) -> Self {
                |a, b| Self::div(a, b as f32)
            }

            fn to_string() -> fn(&mut Self) -> String {
                |x| <Self as ToString>::to_string(x)
            }

            fn to_debug() -> fn(&mut Self) -> String {
                |x| format!("{x:?}")
            }
        }
    };
}

impl_vec!(
    Vec2,
    "vec2",
    |array: Array| -> Result<Self, Box<EvalAltResult>> {
        Ok(Self::new(
            array.get(0).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(1).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
        ))
    }
);

impl_vec!(
    Vec3,
    "vec3",
    |array: Array| -> Result<Self, Box<EvalAltResult>> {
        Ok(Self::new(
            array.get(0).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(1).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(2).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
        ))
    }
);

impl_vec!(
    Vec4,
    "vec4",
    |array: Array| -> Result<Self, Box<EvalAltResult>> {
        Ok(Self::new(
            array.get(0).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(1).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(2).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
            array.get(3).map(Dynamic::as_float).unwrap_or(Ok(0.0))? as f32,
        ))
    }
);
