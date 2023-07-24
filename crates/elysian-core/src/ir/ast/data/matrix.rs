use std::borrow::Cow;

use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, StructDefinition, Type},
};

use super::{VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT};

// New struct-based representation

pub const X_AXIS_2: Property =
    Property::new_primitive("x_axis", Type::Struct(Cow::Borrowed(VECTOR2_STRUCT)));
pub const Y_AXIS_2: Property =
    Property::new_primitive("y_axis", Type::Struct(Cow::Borrowed(VECTOR2_STRUCT)));

pub const X_AXIS_3: Property =
    Property::new_primitive("x_axis", Type::Struct(Cow::Borrowed(VECTOR3_STRUCT)));
pub const Y_AXIS_3: Property =
    Property::new_primitive("y_axis", Type::Struct(Cow::Borrowed(VECTOR3_STRUCT)));
pub const Z_AXIS_3: Property =
    Property::new_primitive("z_axis", Type::Struct(Cow::Borrowed(VECTOR3_STRUCT)));

pub const X_AXIS_4: Property =
    Property::new_primitive("x_axis", Type::Struct(Cow::Borrowed(VECTOR4_STRUCT)));
pub const Y_AXIS_4: Property =
    Property::new_primitive("y_axis", Type::Struct(Cow::Borrowed(VECTOR4_STRUCT)));
pub const Z_AXIS_4: Property =
    Property::new_primitive("z_axis", Type::Struct(Cow::Borrowed(VECTOR4_STRUCT)));
pub const W_AXIS_4: Property =
    Property::new_primitive("w_axis", Type::Struct(Cow::Borrowed(VECTOR4_STRUCT)));

pub const MATRIX2_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X_AXIS_2,
        public: true,
    },
    FieldDefinition {
        prop: Y_AXIS_2,
        public: true,
    },
];

pub const MATRIX2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix2", 0),
    public: false,
    fields: Cow::Borrowed(MATRIX2_FIELDS),
};

pub const MATRIX3_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X_AXIS_3,
        public: true,
    },
    FieldDefinition {
        prop: Y_AXIS_3,
        public: true,
    },
    FieldDefinition {
        prop: Z_AXIS_3,
        public: true,
    },
];

pub const MATRIX3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix3", 0),
    public: false,
    fields: Cow::Borrowed(MATRIX3_FIELDS),
};

pub const MATRIX4_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        prop: X_AXIS_4,
        public: true,
    },
    FieldDefinition {
        prop: Y_AXIS_4,
        public: true,
    },
    FieldDefinition {
        prop: Z_AXIS_4,
        public: true,
    },
    FieldDefinition {
        prop: W_AXIS_4,
        public: true,
    },
];

pub const MATRIX4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix4", 0),
    public: false,
    fields: Cow::Borrowed(MATRIX4_FIELDS),
};
