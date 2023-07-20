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

use super::{Identifier, TypeSpec};

pub const X: Property = Property::new_primitive("x", Type::Number);
pub const Y: Property = Property::new_primitive("y", Type::Number);
pub const Z: Property = Property::new_primitive("z", Type::Number);

pub const POSITION_2D: Property = Property::new("position_2d", Type::Vector2, 19300293251480055481);
pub const POSITION_3D: Property = Property::new("position_3d", Type::Vector3, 2063026210185456313);
pub const TIME: Property = Property::new("time", Type::Number, 391570251245214947);
pub const DISTANCE: Property = Property::new("distance", Type::Number, 20699600731090380932);
pub const GRADIENT_2D: Property = Property::new("gradient_2d", Type::Vector2, 16702807221222221695);
pub const GRADIENT_3D: Property = Property::new("gradient_3d", Type::Vector3, 1183200891820394544);
pub const NORMAL: Property = Property::new("normal", Type::Vector3, 1183200891820394544);
pub const UV: Property = Property::new("uv", Type::Vector2, 1527481748115194786);
pub const TANGENT_2D: Property = Property::new("tangent_2d", Type::Vector2, 12976793731289731131);
pub const TANGENT_3D: Property = Property::new("tangent_3d", Type::Vector3, 17286461381478601027);
pub const COLOR: Property = Property::new("color", Type::Vector4, 84604795624457789);
pub const LIGHT: Property = Property::new("light", Type::Number, 1330409404139204842);
pub const SUPPORT_2D: Property = Property::new("support_2d", Type::Vector2, 85970193295239647);
pub const SUPPORT_3D: Property = Property::new(
    "support_3d",
    Type::Vector3,
    5120220911040556255970193295239647,
);
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

impl IntoIterator for Property {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Property {
    pub const fn new(name: &'static str, ty: Type, uuid: u128) -> Self {
        Property {
            id: Identifier::new(name, uuid),
            ty,
        }
    }

    pub const fn new_primitive(name: &'static str, ty: Type) -> Self {
        Property {
            id: Identifier::new(name, 0),
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

    pub fn read<T>(self) -> Expr<T>
    where
        T: TypeSpec,
    {
        Read(None, vec![self])
    }

    pub fn bind<T>(self, expr: Expr<T>) -> Stmt<T>
    where
        T: TypeSpec,
    {
        Write {
            bind: true,
            path: vec![self],
            expr,
        }
    }

    pub fn write<T>(self, expr: Expr<T>) -> Stmt<T>
    where
        T: TypeSpec,
    {
        Write {
            bind: false,
            path: vec![self],
            expr,
        }
    }
}

impl From<Attribute> for Property {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Position => POSITION_2D,
            Attribute::Time => TIME,
            Attribute::Distance => DISTANCE,
            Attribute::Gradient => GRADIENT_2D,
            Attribute::Uv => UV,
            Attribute::Tangent => TANGENT_2D,
            Attribute::Color => COLOR,
            Attribute::Light => LIGHT,
        }
    }
}

pub trait IntoRead<T>
where
    T: TypeSpec,
{
    fn read(self) -> Expr<T>;
}

impl<T, U> IntoRead<U> for T
where
    U: TypeSpec,
    T: IntoIterator<Item = Property>,
{
    fn read(self) -> Expr<U> {
        Read(None, self.into_iter().collect())
    }
}

pub trait IntoBind<T>
where
    T: TypeSpec,
{
    fn bind(self, expr: Expr<T>) -> Stmt<T>;
}

impl<T, U> IntoBind<U> for T
where
    U: TypeSpec,
    T: IntoIterator<Item = Property>,
{
    fn bind(self, expr: Expr<U>) -> Stmt<U> {
        Write {
            bind: true,
            path: self.into_iter().collect(),
            expr,
        }
    }
}

pub trait IntoWrite<T>
where
    T: TypeSpec,
{
    fn write(self, expr: Expr<T>) -> Stmt<T>;
}

impl<T, U> IntoWrite<U> for T
where
    U: TypeSpec,
    T: IntoIterator<Item = Property>,
{
    fn write(self, expr: Expr<U>) -> Stmt<U> {
        Write {
            bind: false,
            path: self.into_iter().collect(),
            expr,
        }
    }
}
