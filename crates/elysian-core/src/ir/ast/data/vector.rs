use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, StructDefinition, Type},
};

// New Struct-based impl

pub const X: Property = Property::new_primitive("x", Type::Number);
pub const Y: Property = Property::new_primitive("y", Type::Number);
pub const Z: Property = Property::new_primitive("z", Type::Number);
pub const W: Property = Property::new_primitive("w", Type::Number);

pub const VECTOR2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector2", 8915589261187194730),
    public: false,
    fields: &[
        FieldDefinition {
            prop: X,
            public: true,
        },
        FieldDefinition {
            prop: Y,
            public: true,
        },
    ],
};

pub const VECTOR3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector3", 8915589261187194730),
    public: false,
    fields: &[
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
    ],
};

pub const VECTOR4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Vector4", 8915589261187194730),
    public: false,
    fields: &[
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
    ],
};
