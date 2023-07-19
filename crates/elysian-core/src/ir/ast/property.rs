use crate::{
    ast::{attribute::Attribute, combine::COMBINE_CONTEXT_STRUCT, modify::CONTEXT_STRUCT},
    ir::{
        ast::{
            Expr::{self, *},
            Stmt::{self, *},
        },
        module::Type,
    },
};

use super::{Identifier, TypeSpec, VectorSpace};

pub const POSITION: Property = Property::new("position", Type::VectorSpace, 19300293251480055481);
pub const TIME: Property = Property::new("time", Type::Number, 391570251245214947);
pub const DISTANCE: Property = Property::new("distance", Type::Number, 20699600731090380932);
pub const GRADIENT: Property = Property::new("gradient", Type::VectorSpace, 16702807221222221695);
pub const UV: Property = Property::new("uv", Type::Vector2, 1527481748115194786);
pub const TANGENT: Property = Property::new("tangent", Type::VectorSpace, 12976793731289731131);
pub const COLOR: Property = Property::new("color", Type::Vector4, 84604795624457789);
pub const LIGHT: Property = Property::new("light", Type::Number, 1330409404139204842);
pub const SUPPORT: Property = Property::new("support", Type::VectorSpace, 85970193295239647);
pub const ERROR: Property = Property::new("error", Type::Number, 209621851525461471);
pub const NUM: Property = Property::new("num", Type::Number, 1349662877516236181);

pub const CONTEXT: Property =
    Property::new("context", Type::Struct(&CONTEXT_STRUCT), 595454262490629935);
pub const COMBINE_CONTEXT: Property = Property::new(
    "combine_context",
    Type::Struct(&COMBINE_CONTEXT_STRUCT),
    671133652169921634,
);

pub const LEFT: Property = Property::new("left", Type::Struct(&CONTEXT_STRUCT), 635254731934742132);
pub const RIGHT: Property =
    Property::new("right", Type::Struct(&CONTEXT_STRUCT), 5251097991491214179);
pub const OUT: Property = Property::new("out", Type::Struct(&CONTEXT_STRUCT), 1470763158891875334);

/// Named, typed unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    id: Identifier,
    ty: Type,
}

impl Property {
    pub const fn new(name: &'static str, ty: Type, uuid: u128) -> Self {
        Property {
            id: Identifier::new(name, uuid),
            ty,
        }
    }

    pub const fn id(&self) -> &Identifier {
        &self.id
    }

    pub const fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }

    pub fn read<T, const N: usize>(self) -> Expr<T, N>
    where
        T: TypeSpec + VectorSpace<N>,
    {
        Read(None, vec![self])
    }

    pub fn write<T, const N: usize>(self, expr: Expr<T, N>) -> Stmt<T, N>
    where
        T: TypeSpec + VectorSpace<N>,
    {
        Write {
            path: vec![self],
            expr,
        }
    }
}

impl From<Attribute> for Property {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Position => POSITION,
            Attribute::Time => TIME,
            Attribute::Distance => DISTANCE,
            Attribute::Gradient => GRADIENT,
            Attribute::Uv => UV,
            Attribute::Tangent => TANGENT,
            Attribute::Color => COLOR,
            Attribute::Light => LIGHT,
        }
    }
}

pub trait IntoRead<T, const N: usize>: IntoIterator<Item = Property>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn read(self) -> Expr<T, N>;
}

impl<T, U, const N: usize> IntoRead<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: IntoIterator<Item = Property>,
{
    fn read(self) -> Expr<U, N> {
        Read(None, self.into_iter().collect())
    }
}

pub trait IntoWrite<T, const N: usize>: IntoIterator<Item = Property>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn write(self, expr: Expr<T, N>) -> Stmt<T, N>;
}

impl<T, U, const N: usize> IntoWrite<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: IntoIterator<Item = Property>,
{
    fn write(self, expr: Expr<U, N>) -> Stmt<U, N> {
        Write {
            path: self.into_iter().collect(),
            expr,
        }
    }
}
