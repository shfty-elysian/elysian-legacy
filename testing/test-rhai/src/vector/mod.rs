use rhai::{Array, Engine, EvalAltResult, FLOAT, INT};

mod float;

mod glam;
pub use self::glam::*;

pub trait Vector: 'static + Clone + Send + Sync {
    fn name() -> &'static str;

    fn new() -> fn(Array) -> Result<Self, Box<EvalAltResult>>;

    fn splat() -> fn(FLOAT) -> Self;
    fn length() -> fn(Self) -> FLOAT;
    fn normalize() -> fn(Self) -> Self;
    fn abs() -> fn(Self) -> Self;
    fn sign() -> fn(Self) -> Self;
    fn min_vec() -> fn(Self, Self) -> Self;
    fn max_vec() -> fn(Self, Self) -> Self;
    fn index() -> fn(&mut Self, INT) -> FLOAT;

    fn add_vec() -> fn(Self, Self) -> Self;
    fn add_float() -> fn(Self, FLOAT) -> Self;

    fn sub_vec() -> fn(Self, Self) -> Self;
    fn sub_float() -> fn(Self, FLOAT) -> Self;

    fn mul_vec() -> fn(Self, Self) -> Self;
    fn mul_float() -> fn(Self, FLOAT) -> Self;

    fn div_vec() -> fn(Self, Self) -> Self;
    fn div_float() -> fn(Self, FLOAT) -> Self;

    fn to_string() -> fn(&mut Self) -> String;
    fn to_debug() -> fn(&mut Self) -> String;

    /// Register this type with the engine
    fn register(engine: &mut Engine) {
        let name = Self::name();

        engine
            .register_type::<Self>()
            .register_indexer_get(Self::index())
            .register_fn("splat", Self::splat())
            .register_fn("length", Self::length())
            .register_fn("normalize", Self::normalize())
            .register_fn("abs", Self::abs())
            .register_fn("sign", Self::sign())
            .register_fn("min", Self::min_vec())
            .register_fn("max", Self::max_vec())
            .register_fn("+", Self::add_vec())
            .register_fn("+", Self::add_float())
            .register_fn("-", Self::sub_vec())
            .register_fn("-", Self::sub_float())
            .register_fn("*", Self::mul_vec())
            .register_fn("*", Self::mul_float())
            .register_fn("/", Self::div_vec())
            .register_fn("/", Self::div_float())
            .register_fn("to_string", Self::to_string())
            .register_fn("to_debug", Self::to_debug());

        Self::register_new(engine, name);
    }

    /// Register the vector's constructor with the given name
    fn register_new(engine: &mut Engine, name: &'static str) {
        engine.register_fn(name, Self::new());
    }
}
