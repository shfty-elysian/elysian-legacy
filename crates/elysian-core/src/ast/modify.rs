use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use indexmap::IndexMap;

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{
        Identifier, IntoBlock, COLOR, CONTEXT, DISTANCE, ERROR, GRADIENT_2D, GRADIENT_3D, LIGHT,
        NORMAL, POSITION_2D, POSITION_3D, SUPPORT_2D, SUPPORT_3D, TANGENT_2D, TANGENT_3D, TIME, UV,
    },
    module::{
        AsModule, DynAsModule, FieldDefinition, FunctionDefinition, InputDefinition,
        SpecializationData, StructDefinition, Type,
    },
};

pub const CONTEXT_STRUCT_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: POSITION_2D,
        public: true,
    },
    FieldDefinition {
        id: POSITION_3D,
        public: true,
    },
    FieldDefinition {
        id: TIME,
        public: true,
    },
    FieldDefinition {
        id: DISTANCE,
        public: true,
    },
    FieldDefinition {
        id: GRADIENT_2D,
        public: true,
    },
    FieldDefinition {
        id: GRADIENT_3D,
        public: true,
    },
    FieldDefinition {
        id: NORMAL,
        public: true,
    },
    FieldDefinition {
        id: UV,
        public: true,
    },
    FieldDefinition {
        id: TANGENT_2D,
        public: true,
    },
    FieldDefinition {
        id: TANGENT_3D,
        public: true,
    },
    FieldDefinition {
        id: COLOR,
        public: true,
    },
    FieldDefinition {
        id: LIGHT,
        public: true,
    },
    FieldDefinition {
        id: SUPPORT_2D,
        public: true,
    },
    FieldDefinition {
        id: SUPPORT_3D,
        public: true,
    },
    FieldDefinition {
        id: ERROR,
        public: true,
    },
];

pub struct Modify {
    pub pre_modifiers: Vec<DynAsIR>,
    pub field: DynAsModule,
    pub post_modifiers: Vec<DynAsIR>,
}

impl Debug for Modify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Modify")
            .field("pre_modifiers", &self.pre_modifiers)
            .field("field", &self.field)
            .field("post_modifiers", &self.post_modifiers)
            .finish()
    }
}

impl Hash for Modify {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for modifier in &self.pre_modifiers {
            state.write_u64(modifier.hash_ir());
        }
        state.write_u64(self.field.hash_ir());
        for modifier in &self.post_modifiers {
            state.write_u64(modifier.hash_ir());
        }
    }
}

impl AsModule for Modify {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("modify")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        tys: &IndexMap<Identifier, Type>,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        let field_entry_point = self.field.entry_point();
        self.pre_modifiers
            .iter()
            .flat_map(|t| AsIR::functions(t, spec))
            .chain(self.field.functions(spec, tys, &field_entry_point))
            .chain(
                self.post_modifiers
                    .iter()
                    .flat_map(|t| AsIR::functions(t, spec)),
            )
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT,
                block: self
                    .post_modifiers
                    .iter()
                    .fold(
                        field_entry_point.call([self
                            .pre_modifiers
                            .iter()
                            .fold(CONTEXT.read(), |acc, next| next.expression(spec, acc))]),
                        |acc, next| next.expression(spec, acc),
                    )
                    .output()
                    .block(),
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoModify: 'static + Sized + AsModule {
    fn modify(self) -> Modify {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T> IntoModify for T where T: 'static + Sized + AsModule {}
