use std::borrow::Cow;

use crate::ir::{
    ast::{Identifier, Property},
    module::{FieldDefinition, StructDefinition, Type, PROPERTIES},
};

use super::{VECTOR2, VECTOR3, VECTOR4};

// New struct-based representation

pub const X_AXIS_2: Identifier = Identifier::new("x_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static X_AXIS_2_PROP: Property = Property {
    id: X_AXIS_2,
    ty: Type::Struct(VECTOR2),
};

pub const Y_AXIS_2: Identifier = Identifier::new("y_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Y_AXIS_2_PROP: Property = Property {
    id: Y_AXIS_2,
    ty: Type::Struct(VECTOR2),
};

pub const X_AXIS_3: Identifier = Identifier::new("x_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static X_AXIS_3_PROP: Property = Property {
    id: X_AXIS_3,
    ty: Type::Struct(VECTOR3),
};

pub const Y_AXIS_3: Identifier = Identifier::new("y_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Y_AXIS_3_PROP: Property = Property {
    id: Y_AXIS_3,
    ty: Type::Struct(VECTOR3),
};

pub const Z_AXIS_3: Identifier = Identifier::new("z_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Z_AXIS_3_PROP: Property = Property {
    id: Z_AXIS_3,
    ty: Type::Struct(VECTOR3),
};

pub const X_AXIS_4: Identifier = Identifier::new("x_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static X_AXIS_4_PROP: Property = Property {
    id: X_AXIS_4,
    ty: Type::Struct(VECTOR4),
};

pub const Y_AXIS_4: Identifier = Identifier::new("y_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Y_AXIS_4_PROP: Property = Property {
    id: Y_AXIS_4,
    ty: Type::Struct(VECTOR4),
};

pub const Z_AXIS_4: Identifier = Identifier::new("z_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static Z_AXIS_4_PROP: Property = Property {
    id: Z_AXIS_4,
    ty: Type::Struct(VECTOR4),
};

pub const W_AXIS_4: Identifier = Identifier::new("w_axis", 0);
#[linkme::distributed_slice(PROPERTIES)]
static W_AXIS_4_PROP: Property = Property {
    id: W_AXIS_4,
    ty: Type::Struct(VECTOR4),
};

pub const MATRIX2: Identifier = Identifier::new("Matrix2", 0);

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
