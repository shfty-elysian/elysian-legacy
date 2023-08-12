use std::borrow::Cow;

use crate::{
    ast::{identifier::Identifier, property_identifier::PropertyIdentifier},
    ir::module::{FieldDefinition, StructDefinition, StructIdentifier, Type},
    property,
};

use super::{VECTOR2, VECTOR3, VECTOR4};

// New struct-based representation

pub const X_AXIS_2: Identifier = Identifier::new("x_axis", 0);
property!(
    X_AXIS_2,
    X_AXIS_2_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const Y_AXIS_2: Identifier = Identifier::new("y_axis", 0);
property!(
    Y_AXIS_2,
    Y_AXIS_2_PROP,
    Type::Struct(StructIdentifier(VECTOR2))
);

pub const X_AXIS_3: Identifier = Identifier::new("x_axis", 0);
property!(
    X_AXIS_3,
    X_AXIS_3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const Y_AXIS_3: Identifier = Identifier::new("y_axis", 0);
property!(
    Y_AXIS_3,
    Y_AXIS_3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const Z_AXIS_3: Identifier = Identifier::new("z_axis", 0);
property!(
    Z_AXIS_3,
    Z_AXIS_3_PROP,
    Type::Struct(StructIdentifier(VECTOR3))
);

pub const X_AXIS_4: Identifier = Identifier::new("x_axis", 0);
property!(
    X_AXIS_4,
    X_AXIS_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const Y_AXIS_4: Identifier = Identifier::new("y_axis", 0);
property!(
    Y_AXIS_4,
    Y_AXIS_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const Z_AXIS_4: Identifier = Identifier::new("z_axis", 0);
property!(
    Z_AXIS_4,
    Z_AXIS_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const W_AXIS_4: Identifier = Identifier::new("w_axis", 0);
property!(
    W_AXIS_4,
    W_AXIS_4_PROP,
    Type::Struct(StructIdentifier(VECTOR4))
);

pub const MATRIX2: Identifier = Identifier::new("Matrix2", 0);
property!(
    MATRIX2,
    MATRIX2_PROP,
    Type::Struct(StructIdentifier(MATRIX2))
);

pub const MATRIX2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X_AXIS_2),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y_AXIS_2),
        public: true,
    },
];

pub const MATRIX2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(MATRIX2),
    public: false,
    fields: Cow::Borrowed(MATRIX2_FIELDS),
};

pub const MATRIX3: Identifier = Identifier::new("Matrix3", 0);
property!(
    MATRIX3,
    MATRIX3_PROP,
    Type::Struct(StructIdentifier(MATRIX3))
);

pub const MATRIX3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X_AXIS_3),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y_AXIS_3),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Z_AXIS_3),
        public: true,
    },
];

pub const MATRIX3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(MATRIX3),
    public: false,
    fields: Cow::Borrowed(MATRIX3_FIELDS),
};

pub const MATRIX4: Identifier = Identifier::new("Matrix4", 0);
property!(
    MATRIX4,
    MATRIX4_PROP,
    Type::Struct(StructIdentifier(MATRIX4))
);

pub const MATRIX4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(X_AXIS_4),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Y_AXIS_4),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(Z_AXIS_4),
        public: true,
    },
    FieldDefinition {
        id: PropertyIdentifier(W_AXIS_4),
        public: true,
    },
];

pub const MATRIX4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(MATRIX4),
    public: false,
    fields: Cow::Borrowed(MATRIX4_FIELDS),
};
