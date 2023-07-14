use crate::{
    elysian::attribute::Attribute,
    ir::{
        ast::{
            Expr::{self, *},
            Stmt::{self, *},
        },
        module::Type,
    },
};

use super::Identifier;

pub const POSITION: Property = Property::new("position", Type::Vector, 19300293251480055481);
pub const TIME: Property = Property::new("time", Type::Number, 391570251245214947);
pub const DISTANCE: Property = Property::new("distance", Type::Number, 20699600731090380932);
pub const GRADIENT: Property = Property::new("gradient", Type::Vector, 16702807221222221695);
pub const UV: Property = Property::new("uv", Type::Vector, 1527481748115194786);
pub const TANGENT: Property = Property::new("tangent", Type::Vector, 12976793731289731131);
pub const COLOR: Property = Property::new("color", Type::Vector, 84604795624457789);
pub const LIGHT: Property = Property::new("light", Type::Number, 1330409404139204842);
pub const SUPPORT: Property = Property::new("support", Type::Vector, 85970193295239647);
pub const ERROR: Property = Property::new("error", Type::Number, 209621851525461471);
pub const K: Property = Property::new("k", Type::Number, 12632115441234896764);
pub const NUM: Property = Property::new("num", Type::Number, 1349662877516236181);
pub const VECT: Property = Property::new("vect", Type::Vector, 19553087511741435087);
pub const CONTEXT: Property = Property::new("context", Type::Struct("Context"), 595454262490629935);
pub const COMBINE_CONTEXT: Property = Property::new(
    "combine_context",
    Type::Struct("CombineContext"),
    671133652169921634,
);
pub const LEFT: Property = Property::new("left", Type::Struct("Context"), 635254731934742132);
pub const RIGHT: Property = Property::new("right", Type::Struct("Context"), 5251097991491214179);
pub const OUT: Property = Property::new("out", Type::Struct("Context"), 1470763158891875334);

/// Block variable names
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Property {
    pub fn name(&self) -> &'static str {
        self.id.name()
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

pub trait IntoRead<N, V>: IntoIterator<Item = Property> {
    fn read(self) -> Expr<N, V>;
}

impl<T, N, V> IntoRead<N, V> for T
where
    T: IntoIterator<Item = Property>,
{
    fn read(self) -> Expr<N, V> {
        Read(self.into_iter().collect())
    }
}

pub trait IntoWrite<N, V>: IntoIterator<Item = Property> {
    fn write(self, expr: Expr<N, V>) -> Stmt<N, V>;
}

impl<T, N, V> IntoWrite<N, V> for T
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
