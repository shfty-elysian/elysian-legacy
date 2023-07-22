use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, StructDefinition, Type},
};

use super::{VECTOR2_STRUCT, VECTOR3_STRUCT, VECTOR4_STRUCT};

// New struct-based representation

pub const X_AXIS_2: Property = Property::new_primitive("x_axis", Type::Struct(&VECTOR2_STRUCT));
pub const Y_AXIS_2: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR2_STRUCT));

pub const X_AXIS_3: Property = Property::new_primitive("x_axis", Type::Struct(&VECTOR3_STRUCT));
pub const Y_AXIS_3: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR3_STRUCT));
pub const Z_AXIS_3: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR3_STRUCT));

pub const X_AXIS_4: Property = Property::new_primitive("x_axis", Type::Struct(&VECTOR4_STRUCT));
pub const Y_AXIS_4: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR4_STRUCT));
pub const Z_AXIS_4: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR4_STRUCT));
pub const W_AXIS_4: Property = Property::new_primitive("y_axis", Type::Struct(&VECTOR4_STRUCT));

pub const MATRIX2_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix2", 20261747501874881448),
    public: false,
    fields: &[
        FieldDefinition {
            prop: X_AXIS_2,
            public: true,
        },
        FieldDefinition {
            prop: Y_AXIS_2,
            public: true,
        },
    ],
};

pub const MATRIX3_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix3", 7315148181882206775),
    public: false,
    fields: &[
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
    ],
};

pub const MATRIX4_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Matrix4", 202137800871303460),
    public: false,
    fields: &[
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
    ],
};
