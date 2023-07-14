use crate::{
    elysian::attribute::Attribute,
    ir::ast::{
        Expr::{self, *},
        Stmt::{self, *},
    },
};

/// Block variable names
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Property {
    Position,
    Time,
    Distance,
    Gradient,
    Uv,
    Tangent,
    Color,
    Light,
    Support,
    Error,
    Bool,
    Num,
    Vect,
    K,
    Context,
    CombineContext,
    Left,
    Right,
    Out,
}

impl From<Attribute> for Property {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Position => Property::Position,
            Attribute::Time => Property::Time,
            Attribute::Distance => Property::Distance,
            Attribute::Gradient => Property::Gradient,
            Attribute::Uv => Property::Uv,
            Attribute::Tangent => Property::Tangent,
            Attribute::Color => Property::Color,
            Attribute::Light => Property::Light,
        }
    }
}

impl Property {
    pub fn name(&self) -> &'static str {
        match self {
            Property::Position => "position",
            Property::Time => "time",
            Property::Distance => "distance",
            Property::Gradient => "gradient",
            Property::Uv => "uv",
            Property::Tangent => "tangent",
            Property::Color => "color",
            Property::Light => "light",
            Property::Support => "support",
            Property::Error => "error",
            Property::Bool => "bool",
            Property::Num => "num",
            Property::Vect => "vect",
            Property::K => "k",
            Property::Context => "context",
            Property::CombineContext => "combine_context",
            Property::Left => "left",
            Property::Right => "right",
            Property::Out => "out",
        }
    }

    pub fn read<N, V>(self) -> Expr<N, V> {
        Read(vec![self])
    }

    pub fn write<N, V>(self, expr: Expr<N, V>) -> Stmt<N, V> {
        Write {
            path: vec![self],
            expr,
        }
    }
}

pub trait IntoPathRead<N, V>: IntoIterator<Item = Property> {
    fn read(self) -> Expr<N, V>;
}

impl<T, N, V> IntoPathRead<N, V> for T
where
    T: IntoIterator<Item = Property>,
{
    fn read(self) -> Expr<N, V> {
        Read(self.into_iter().collect())
    }
}

pub trait IntoPathWrite<N, V>: IntoIterator<Item = Property> {
    fn write(self, expr: Expr<N, V>) -> Stmt<N, V>;
}

impl<T, N, V> IntoPathWrite<N, V> for T
where
    T: IntoIterator<Item = Property>,
{
    fn write(self, expr: Expr<N, V>) -> Stmt<N, V> {
        Write {
            path: self.into_iter().collect(),
            expr,
        }
    }
}
