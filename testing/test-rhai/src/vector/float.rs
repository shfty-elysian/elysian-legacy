use rhai::{Array, Dynamic, EvalAltResult, FLOAT, INT};
use std::ops::{Add, Div, Mul, Sub};

use crate::Vector;

impl Vector for FLOAT {
    fn name() -> &'static str {
        "vec1"
    }

    fn new() -> fn(Array) -> Result<Self, Box<EvalAltResult>> {
        |x| Ok(x.get(0).map(Dynamic::as_float).unwrap_or(Ok(0.0))?)
    }

    fn index() -> fn(&mut Self, INT) -> FLOAT {
        |x, idx| {
            if idx < 0 || idx >= 1 {
                Default::default()
            } else {
                *x
            }
        }
    }

    fn splat() -> fn(FLOAT) -> Self {
        |x| x
    }

    fn length() -> fn(Self) -> FLOAT {
        |x| x
    }

    fn normalize() -> fn(Self) -> Self {
        FLOAT::signum
    }

    fn abs() -> fn(Self) -> Self {
        FLOAT::abs
    }

    fn sign() -> fn(Self) -> Self {
        FLOAT::signum
    }

    fn min_vec() -> fn(Self, Self) -> Self {
        FLOAT::min
    }

    fn max_vec() -> fn(Self, Self) -> Self {
        FLOAT::max
    }

    fn add_vec() -> fn(Self, Self) -> Self {
        FLOAT::add
    }

    fn add_float() -> fn(Self, FLOAT) -> Self {
        FLOAT::add
    }

    fn sub_vec() -> fn(Self, Self) -> Self {
        FLOAT::sub
    }

    fn sub_float() -> fn(Self, FLOAT) -> Self {
        FLOAT::sub
    }

    fn mul_vec() -> fn(Self, Self) -> Self {
        FLOAT::mul
    }

    fn mul_float() -> fn(Self, FLOAT) -> Self {
        FLOAT::mul
    }

    fn div_vec() -> fn(Self, Self) -> Self {
        FLOAT::div
    }

    fn div_float() -> fn(Self, FLOAT) -> Self {
        FLOAT::div
    }

    fn to_string() -> fn(&mut Self) -> String {
        |x| x.to_string()
    }

    fn to_debug() -> fn(&mut Self) -> String {
        |x| format!("{:?}", x)
    }
}
