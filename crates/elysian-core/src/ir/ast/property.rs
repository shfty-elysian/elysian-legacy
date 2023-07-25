use std::fmt::Display;

use crate::{
    ir::module::{NumericType, PropertyIdentifier, StructIdentifier, Type},
    property,
};

use super::{Identifier, VECTOR2, VECTOR3, VECTOR4};

pub const POSITION_2D: PropertyIdentifier =
    PropertyIdentifier::new("position_2d", 19300293251480055481);
property!(
    POSITION_2D,
    POSITION_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const POSITION_3D: PropertyIdentifier =
    PropertyIdentifier::new("position_3d", 2063026210185456313);
property!(
    POSITION_3D,
    POSITION_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const TIME: PropertyIdentifier = PropertyIdentifier::new("time", 391570251245214947);
property!(TIME, TIME_PROP, Type::Number(NumericType::Float));

pub const DISTANCE: PropertyIdentifier = PropertyIdentifier::new("distance", 20699600731090380932);
property!(DISTANCE, DISTANCE_PROP, Type::Number(NumericType::Float));

pub const GRADIENT_2D: PropertyIdentifier =
    PropertyIdentifier::new("gradient_2d", 16702807221222221695);
property!(
    GRADIENT_2D,
    GRADIENT_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const GRADIENT_3D: PropertyIdentifier =
    PropertyIdentifier::new("gradient_3d", 1282963704979353552);
property!(
    GRADIENT_3D,
    GRADIENT_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const NORMAL: PropertyIdentifier = PropertyIdentifier::new("normal", 1183200891820394544);
property!(NORMAL, NORMAL_PROP, Type::Struct(StructIdentifier(VECTOR3)));

pub const UV: PropertyIdentifier = PropertyIdentifier::new("uv", 1527481748115194786);
property!(UV, UV_PROP, Type::Struct(StructIdentifier(VECTOR2)));

pub const TANGENT_2D: PropertyIdentifier =
    PropertyIdentifier::new("tangent_2d", 12976793731289731131);
property!(
    TANGENT_2D,
    TANGENT_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const TANGENT_3D: PropertyIdentifier =
    PropertyIdentifier::new("tangent_3d", 17286461381478601027);
property!(
    TANGENT_3D,
    TANGENT_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const COLOR: PropertyIdentifier = PropertyIdentifier::new("color", 84604795624457789);
property!(COLOR, COLOR_PROP, Type::Struct(StructIdentifier(VECTOR4)));

pub const LIGHT: PropertyIdentifier = PropertyIdentifier::new("light", 1330409404139204842);
property!(LIGHT, LIGHT_PROP, Type::Number(NumericType::Float));

pub const SUPPORT_2D: PropertyIdentifier = PropertyIdentifier::new("support_2d", 85970193295239647);
property!(
    SUPPORT_2D,
    SUPPORT_2D_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const SUPPORT_3D: PropertyIdentifier =
    PropertyIdentifier::new("support_3d", 5120220911040556255970193295239647);
property!(
    SUPPORT_3D,
    SUPPORT_3D_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const ERROR: PropertyIdentifier = PropertyIdentifier::new("error", 209621851525461471);
property!(ERROR, ERROR_PROP, Type::Number(NumericType::Float));

pub const NUM: PropertyIdentifier = PropertyIdentifier::new("num", 1349662877516236181);
property!(NUM, NUM_PROP, Type::Number(NumericType::Float));

pub const COMBINE_CONTEXT: Identifier = Identifier::new("CombineContext", 671133652169921634);
pub const COMBINE_CONTEXT_PROP: PropertyIdentifier = PropertyIdentifier(COMBINE_CONTEXT);

property!(
    COMBINE_CONTEXT_PROP,
    COMBINE_CONTEXT_PROP_DEF,
    Type::Struct(StructIdentifier(COMBINE_CONTEXT))
);

/// Named, typed unique identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Property {
    pub id: PropertyIdentifier,
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
            id: PropertyIdentifier::new(name, uuid),
            ty,
        }
    }

    pub const fn new_primitive(name: &'static str, ty: Type) -> Self {
        Property {
            id: PropertyIdentifier::new(name, 0),
            ty,
        }
    }

    pub const fn id(&self) -> &PropertyIdentifier {
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
