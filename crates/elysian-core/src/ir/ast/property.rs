use std::fmt::Display;

use crate::ir::{
    ast::{
        Expr::{self, *},
        Stmt::{self, *},
    },
    module::{NumericType, Type, PROPERTIES},
};

use super::{Identifier, VECTOR2, VECTOR3, VECTOR4};

pub const POSITION_2D: Identifier = Identifier::new("position_2d", 19300293251480055481);
#[linkme::distributed_slice(PROPERTIES)]
static POSITION_2D_PROP: Property = Property {
    id: POSITION_2D,
    ty: Type::Struct(VECTOR2),
};

pub const POSITION_3D: Identifier = Identifier::new("position_3d", 2063026210185456313);
#[linkme::distributed_slice(PROPERTIES)]
static POSITION_3D_PROP: Property = Property {
    id: POSITION_3D,
    ty: Type::Struct(VECTOR3),
};

pub const TIME: Identifier = Identifier::new("time", 391570251245214947);
#[linkme::distributed_slice(PROPERTIES)]
static TIME_PROP: Property = Property {
    id: TIME,
    ty: Type::Number(NumericType::Float),
};

pub const DISTANCE: Identifier = Identifier::new("distance", 20699600731090380932);
#[linkme::distributed_slice(PROPERTIES)]
static DISTANCE_PROP: Property = Property {
    id: DISTANCE,
    ty: Type::Number(NumericType::Float),
};

pub const GRADIENT_2D: Identifier = Identifier::new("gradient_2d", 16702807221222221695);
#[linkme::distributed_slice(PROPERTIES)]
static GRADIENT_2D_PROP: Property = Property {
    id: GRADIENT_2D,
    ty: Type::Struct(VECTOR2),
};

pub const GRADIENT_3D: Identifier = Identifier::new("gradient_3d", 1282963704979353552);
#[linkme::distributed_slice(PROPERTIES)]
static GRADIENT_3D_PROP: Property = Property {
    id: GRADIENT_3D,
    ty: Type::Struct(VECTOR3),
};

pub const NORMAL: Identifier = Identifier::new("normal", 1183200891820394544);
#[linkme::distributed_slice(PROPERTIES)]
static NORMAL_PROP: Property = Property {
    id: NORMAL,
    ty: Type::Struct(VECTOR3),
};

pub const UV: Identifier = Identifier::new("uv", 1527481748115194786);
#[linkme::distributed_slice(PROPERTIES)]
static UV_PROP: Property = Property {
    id: UV,
    ty: Type::Struct(VECTOR2),
};

pub const TANGENT_2D: Identifier = Identifier::new("tangent_2d", 12976793731289731131);
#[linkme::distributed_slice(PROPERTIES)]
static TANGENT_2D_PROP: Property = Property {
    id: TANGENT_2D,
    ty: Type::Struct(VECTOR2),
};

pub const TANGENT_3D: Identifier = Identifier::new("tangent_3d", 17286461381478601027);
#[linkme::distributed_slice(PROPERTIES)]
static TANGENT_3D_PROP: Property = Property {
    id: TANGENT_3D,
    ty: Type::Struct(VECTOR3),
};

pub const COLOR: Identifier = Identifier::new("color", 84604795624457789);
#[linkme::distributed_slice(PROPERTIES)]
static COLOR_PROP: Property = Property {
    id: COLOR,
    ty: Type::Struct(VECTOR4),
};

pub const LIGHT: Identifier = Identifier::new("light", 1330409404139204842);
#[linkme::distributed_slice(PROPERTIES)]
static LIGHT_PROP: Property = Property {
    id: LIGHT,
    ty: Type::Number(NumericType::Float),
};

pub const SUPPORT_2D: Identifier = Identifier::new("support_2d", 85970193295239647);
#[linkme::distributed_slice(PROPERTIES)]
static SUPPORT_2D_PROP: Property = Property {
    id: SUPPORT_2D,
    ty: Type::Struct(VECTOR2),
};

pub const SUPPORT_3D: Identifier =
    Identifier::new("support_3d", 5120220911040556255970193295239647);
#[linkme::distributed_slice(PROPERTIES)]
static SUPPORT_3D_PROP: Property = Property {
    id: SUPPORT_3D,
    ty: Type::Struct(VECTOR3),
};

pub const ERROR: Identifier = Identifier::new("error", 209621851525461471);
#[linkme::distributed_slice(PROPERTIES)]
static ERROR_PROP: Property = Property {
    id: ERROR,
    ty: Type::Number(NumericType::Float),
};

pub const NUM: Identifier = Identifier::new("num", 1349662877516236181);
#[linkme::distributed_slice(PROPERTIES)]
static NUM_PROP: Property = Property {
    id: NUM,
    ty: Type::Number(NumericType::Float),
};

pub const CONTEXT: Identifier = Identifier::new("Context", 595454262490629935);
#[linkme::distributed_slice(PROPERTIES)]
static CONTEXT_PROP: Property = Property {
    id: CONTEXT,
    ty: Type::Struct(CONTEXT),
};

pub const COMBINE_CONTEXT: Identifier = Identifier::new("CombineContext", 671133652169921634);
#[linkme::distributed_slice(PROPERTIES)]
static COMBINE_CONTEXT_PROP: Property = Property {
    id: COMBINE_CONTEXT,
    ty: Type::Struct(COMBINE_CONTEXT),
};

/// Named, typed unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    pub id: Identifier,
    pub ty: Type,
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
}

pub trait IntoRead {
    fn read(self) -> Expr;
}

impl<T> IntoRead for T
where
    T: IntoIterator<Item = Identifier>,
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
    T: IntoIterator<Item = Identifier>,
{
    fn write(self, expr: Expr) -> Stmt {
        Write {
            path: self.into_iter().collect(),
            expr,
        }
    }
}
