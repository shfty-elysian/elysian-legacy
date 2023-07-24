use std::fmt::Display;

use crate::{
    ast::{combine::COMBINE_CONTEXT_STRUCT, modify::CONTEXT_STRUCT},
    ir::{
        ast::{
            Expr::{self, *},
            Stmt::{self, *},
        },
        module::{NumericType, Type},
    },
};

use super::{Identifier, VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT};

pub const POSITION_2D: Property = Property::new(
    "position_2d",
    Type::Struct(VECTOR2_STRUCT),
    19300293251480055481,
);
pub const POSITION_3D: Property = Property::new(
    "position_3d",
    Type::Struct(VECTOR3_STRUCT),
    2063026210185456313,
);
pub const TIME: Property =
    Property::new("time", Type::Number(NumericType::Float), 391570251245214947);
pub const DISTANCE: Property = Property::new(
    "distance",
    Type::Number(NumericType::Float),
    20699600731090380932,
);
pub const GRADIENT_2D: Property = Property::new(
    "gradient_2d",
    Type::Struct(VECTOR2_STRUCT),
    16702807221222221695,
);
pub const GRADIENT_3D: Property = Property::new(
    "gradient_3d",
    Type::Struct(VECTOR3_STRUCT),
    1183200891820394544,
);
pub const NORMAL: Property =
    Property::new("normal", Type::Struct(VECTOR3_STRUCT), 1183200891820394544);
pub const UV: Property = Property::new("uv", Type::Struct(VECTOR2_STRUCT), 1527481748115194786);
pub const TANGENT_2D: Property = Property::new(
    "tangent_2d",
    Type::Struct(VECTOR2_STRUCT),
    12976793731289731131,
);
pub const TANGENT_3D: Property = Property::new(
    "tangent_3d",
    Type::Struct(VECTOR3_STRUCT),
    17286461381478601027,
);
pub const COLOR: Property = Property::new("color", Type::Struct(VECTOR4_STRUCT), 84604795624457789);
pub const LIGHT: Property = Property::new(
    "light",
    Type::Number(NumericType::Float),
    1330409404139204842,
);
pub const SUPPORT_2D: Property = Property::new(
    "support_2d",
    Type::Struct(VECTOR2_STRUCT),
    85970193295239647,
);
pub const SUPPORT_3D: Property = Property::new(
    "support_3d",
    Type::Struct(VECTOR3_STRUCT),
    5120220911040556255970193295239647,
);
pub const ERROR: Property = Property::new(
    "error",
    Type::Number(NumericType::Float),
    209621851525461471,
);
pub const NUM: Property =
    Property::new("num", Type::Number(NumericType::Float), 1349662877516236181);

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

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id.name())
    }
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

    pub fn read(self) -> Expr {
        Read(vec![self])
    }

    pub fn bind(self, expr: Expr) -> Stmt {
        Bind { prop: self, expr }
    }

    pub fn write(self, expr: Expr) -> Stmt {
        Write {
            path: vec![self],
            expr,
        }
    }
}

pub trait IntoRead {
    fn read(self) -> Expr;
}

impl<T> IntoRead for T
where
    T: IntoIterator<Item = Property>,
{
    fn read(self) -> Expr {
        Read(self.into_iter().collect())
    }
}

pub trait IntoWrite {
    fn write(self, expr: Expr) -> Stmt;
}

impl<T> IntoWrite for T
where
    T: IntoIterator<Item = Property>,
{
    fn write(self, expr: Expr) -> Stmt {
        Write {
            path: self.into_iter().collect(),
            expr,
        }
    }
}
