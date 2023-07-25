use std::borrow::Cow;

use crate::{
    ir::{
        ast::Identifier,
        module::{FieldDefinition, PropertyIdentifier, StructDefinition, Type},
    },
    property,
};

use super::{VECTOR2, VECTOR3, VECTOR4};

// New struct-based representation

pub const X_AXIS_2: PropertyIdentifier = PropertyIdentifier::new("x_axis", 0);
property!(X_AXIS_2, X_AXIS_2_PROP, Type::Struct(VECTOR2));

pub const Y_AXIS_2: PropertyIdentifier = PropertyIdentifier::new("y_axis", 0);
property!(Y_AXIS_2, Y_AXIS_2_PROP, Type::Struct(VECTOR2));

pub const X_AXIS_3: PropertyIdentifier = PropertyIdentifier::new("x_axis", 0);
property!(X_AXIS_3, X_AXIS_3_PROP, Type::Struct(VECTOR3));

pub const Y_AXIS_3: PropertyIdentifier = PropertyIdentifier::new("y_axis", 0);
property!(Y_AXIS_3, Y_AXIS_3_PROP, Type::Struct(VECTOR3));

pub const Z_AXIS_3: PropertyIdentifier = PropertyIdentifier::new("z_axis", 0);
property!(Z_AXIS_3, Z_AXIS_3_PROP, Type::Struct(VECTOR3));

pub const X_AXIS_4: PropertyIdentifier = PropertyIdentifier::new("x_axis", 0);
property!(X_AXIS_4, X_AXIS_4_PROP, Type::Struct(VECTOR4));

pub const Y_AXIS_4: PropertyIdentifier = PropertyIdentifier::new("y_axis", 0);
property!(Y_AXIS_4, Y_AXIS_4_PROP, Type::Struct(VECTOR4));

pub const Z_AXIS_4: PropertyIdentifier = PropertyIdentifier::new("z_axis", 0);
property!(Z_AXIS_4, Z_AXIS_4_PROP, Type::Struct(VECTOR4));

pub const W_AXIS_4: PropertyIdentifier = PropertyIdentifier::new("w_axis", 0);
property!(W_AXIS_4, W_AXIS_4_PROP, Type::Struct(VECTOR4));


pub const MATRIX2: Identifier = Identifier::new("Matrix2", 0);
pub const MATRIX2_PROP: PropertyIdentifier = PropertyIdentifier(MATRIX2);
property!(MATRIX2_PROP, MATRIX2_PROP_DEF, Type::Struct(MATRIX2));

pub const MATRIX2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X_AXIS_2,
        public: true,
    },
    FieldDefinition {
        id: Y_AXIS_2,
        public: true,
    },
];

pub const MATRIX2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: MATRIX2,
    public: false,
    fields: Cow::Borrowed(MATRIX2_FIELDS),
};

pub const MATRIX3: Identifier = Identifier::new("Matrix3", 0);
pub const MATRIX3_PROP: PropertyIdentifier = PropertyIdentifier(MATRIX3);
property!(MATRIX3_PROP, MATRIX3_PROP_DEF, Type::Struct(MATRIX3));

pub const MATRIX3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X_AXIS_3,
        public: true,
    },
    FieldDefinition {
        id: Y_AXIS_3,
        public: true,
    },
    FieldDefinition {
        id: Z_AXIS_3,
        public: true,
    },
];

pub const MATRIX3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: MATRIX3,
    public: false,
    fields: Cow::Borrowed(MATRIX3_FIELDS),
};

pub const MATRIX4: Identifier = Identifier::new("Matrix4", 0);
pub const MATRIX4_PROP: PropertyIdentifier = PropertyIdentifier(MATRIX4);
property!(MATRIX4_PROP, MATRIX4_PROP_DEF, Type::Struct(MATRIX4));

pub const MATRIX4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: X_AXIS_4,
        public: true,
    },
    FieldDefinition {
        id: Y_AXIS_4,
        public: true,
    },
    FieldDefinition {
        id: Z_AXIS_4,
        public: true,
    },
    FieldDefinition {
        id: W_AXIS_4,
        public: true,
    },
];

pub const MATRIX4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: MATRIX4,
    public: false,
    fields: Cow::Borrowed(MATRIX4_FIELDS),
};
