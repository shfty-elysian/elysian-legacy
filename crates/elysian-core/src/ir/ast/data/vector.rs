use std::borrow::Cow;

use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, NumericType, StructDefinition, Type},
};

// New Struct-based impl

pub const X: Property = Property::new_primitive("x", Type::Number(NumericType::Float));
pub const Y: Property = Property::new_primitive("y", Type::Number(NumericType::Float));
pub const Z: Property = Property::new_primitive("z", Type::Number(NumericType::Float));
pub const W: Property = Property::new_primitive("w", Type::Number(NumericType::Float));

pub const VECTOR2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X,
        public: true,
    },
    FieldDefinition {
        prop: Y,
        public: true,
    },
];

pub const VECTOR2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector2", 0),
    public: false,
    fields: Cow::Borrowed(VECTOR2_FIELDS),
};

pub const VECTOR3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X,
        public: true,
    },
    FieldDefinition {
        prop: Y,
        public: true,
    },
    FieldDefinition {
        prop: Z,
        public: true,
    },
];

pub const VECTOR3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector3", 0),
    public: false,
    fields: Cow::Borrowed(VECTOR3_FIELDS),
};

pub const VECTOR4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X,
        public: true,
    },
    FieldDefinition {
        prop: Y,
        public: true,
    },
    FieldDefinition {
        prop: Z,
        public: true,
    },
    FieldDefinition {
        prop: W,
        public: true,
    },
];

pub const VECTOR4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector4", 0),
    public: false,
    fields: Cow::Borrowed(VECTOR4_FIELDS),
};
