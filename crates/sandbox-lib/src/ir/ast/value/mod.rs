mod into_value;
mod number;
mod structure;
mod vector;

pub use into_value::*;
pub use number::*;
pub use structure::*;
pub use vector::*;

/// Concrete value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Value<N, V> {
    Boolean(bool),
    Number(N),
    Vector(V),
    Struct(Struct<N, V>),
}

impl<N, V> From<crate::elysian::value::Value<N, V>> for Value<N, V> {
    fn from(value: crate::elysian::value::Value<N, V>) -> Self {
        match value {
            crate::elysian::value::Value::Number(n) => Value::Number(n),
            crate::elysian::value::Value::Vector(v) => Value::Vector(v),
        }
    }
}

impl<N, V> From<Box<crate::elysian::value::Value<N, V>>> for Box<Value<N, V>> {
    fn from(value: Box<crate::elysian::value::Value<N, V>>) -> Self {
        Box::new(Value::from(*value))
    }
}
