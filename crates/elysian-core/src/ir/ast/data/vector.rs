use std::borrow::Cow;

use crate::{
    ir::{
        ast::Identifier,
        module::{FieldDefinition, NumericType, PropertyIdentifier, StructDefinition, Type},
    },
    property,
};

pub const X: PropertyIdentifier = PropertyIdentifier::new("x", 0);
property!(X, X_PROP, Type::Number(NumericType::Float));

pub const Y: PropertyIdentifier = PropertyIdentifier::new("y", 0);
property!(Y, Y_PROP, Type::Number(NumericType::Float));

pub const Z: PropertyIdentifier = PropertyIdentifier::new("z", 0);
property!(Z, Z_PROP, Type::Number(NumericType::Float));

pub const W: PropertyIdentifier = PropertyIdentifier::new("w", 0);
property!(W, W_PROP, Type::Number(NumericType::Float));

pub const VECTOR2: Identifier = Identifier::new("Vector2", 0);
pub const VECTOR2_PROP: PropertyIdentifier = PropertyIdentifier(VECTOR2);
property!(VECTOR2_PROP, VECTOR2_PROP_DEF, Type::Struct(VECTOR2));

pub const VECTOR2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X,
        public: true,
    },
    FieldDefinition {
        id: Y,
        public: true,
    },
];

pub const VECTOR2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: VECTOR2,
    public: false,
    fields: Cow::Borrowed(VECTOR2_FIELDS),
};

pub const VECTOR3: Identifier = Identifier::new("Vector3", 0);
pub const VECTOR3_PROP: PropertyIdentifier = PropertyIdentifier(VECTOR3);
property!(VECTOR3_PROP, VECTOR3_PROP_DEF, Type::Struct(VECTOR3));

pub const VECTOR3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X,
        public: true,
    },
    FieldDefinition {
        id: Y,
        public: true,
    },
    FieldDefinition {
        id: Z,
        public: true,
    },
];

pub const VECTOR3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: VECTOR3,
    public: false,
    fields: Cow::Borrowed(VECTOR3_FIELDS),
};

pub const VECTOR4: Identifier = Identifier::new("Vector4", 0);
pub const VECTOR4_PROP: PropertyIdentifier = PropertyIdentifier(VECTOR4);
property!(VECTOR4_PROP, VECTOR4_PROP_DEF, Type::Struct(VECTOR4));

pub const VECTOR4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X,
        public: true,
    },
    FieldDefinition {
        id: Y,
        public: true,
    },
    FieldDefinition {
        id: Z,
        public: true,
    },
    FieldDefinition {
        id: W,
        public: true,
    },
];

pub const VECTOR4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: VECTOR4,
    public: false,
    fields: Cow::Borrowed(VECTOR4_FIELDS),
};
