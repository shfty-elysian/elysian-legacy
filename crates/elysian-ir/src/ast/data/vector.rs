use std::borrow::Cow;

use crate::{
    module::{FieldDefinition, NumericType, StructDefinition, StructIdentifier, Type},
    property,
};
use elysian_core::{identifier::Identifier, property_identifier::PropertyIdentifier};

pub const X: Identifier = Identifier::new("x", 0);
property!(X, X_PROP, Type::Number(NumericType::Float));

pub const Y: Identifier = Identifier::new("y", 0);
property!(Y, Y_PROP, Type::Number(NumericType::Float));

pub const Z: Identifier = Identifier::new("z", 0);
property!(Z, Z_PROP, Type::Number(NumericType::Float));

pub const W: Identifier = Identifier::new("w", 0);
property!(W, W_PROP, Type::Number(NumericType::Float));

pub const VECTOR2: Identifier = Identifier::new("Vector2", 0);
property!(
    VECTOR2,
    VECTOR2_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const VECTOR2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y),
        public: true,
    },
];

pub const VECTOR2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(VECTOR2),
    public: false,
    fields: Cow::Borrowed(VECTOR2_FIELDS),
};

pub const VECTOR3: Identifier = Identifier::new("Vector3", 0);
property!(
    VECTOR3,
    VECTOR3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const VECTOR3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Z),
        public: true,
    },
];

pub const VECTOR3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(VECTOR3),
    public: false,
    fields: Cow::Borrowed(VECTOR3_FIELDS),
};

pub const VECTOR4: Identifier = Identifier::new("Vector4", 0);
property!(
    VECTOR4,
    VECTOR4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const VECTOR4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Z),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(W),
        public: true,
    },
];

pub const VECTOR4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(VECTOR4),
    public: false,
    fields: Cow::Borrowed(VECTOR4_FIELDS),
};
