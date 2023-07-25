use std::borrow::Cow;

use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, NumericType, StructDefinition, Type, PROPERTIES},
};

pub const X: Identifier = Identifier::new("x", 0);
#[linkme::distributed_slice(PROPERTIES)]
static X_PROP: Property = Property {
    id: X,
    ty: Type::Number(NumericType::Float),
};

pub const Y: Identifier = Identifier::new("y", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Y_PROP: Property = Property {
    id: Y,
    ty: Type::Number(NumericType::Float),
};

pub const Z: Identifier = Identifier::new("z", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Z_PROP: Property = Property {
    id: Z,
    ty: Type::Number(NumericType::Float),
};

pub const W: Identifier = Identifier::new("w", 0);
#[linkme::distributed_slice(PROPERTIES)]
static W_PROP: Property = Property {
    id: W,
    ty: Type::Number(NumericType::Float),
};

pub const VECTOR2: Identifier = Identifier::new("Vector2", 0);

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
