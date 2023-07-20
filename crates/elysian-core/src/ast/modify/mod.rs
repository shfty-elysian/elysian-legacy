pub mod post_modifier;
pub mod pre_modifier;

pub use post_modifier::*;
pub use pre_modifier::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{
        Identifier, IntoBlock, IntoValue, TypeSpec, COLOR, CONTEXT, DISTANCE, ERROR, GRADIENT_2D,
        GRADIENT_3D, LIGHT, NORMAL, POSITION_2D, POSITION_3D, SUPPORT_2D, SUPPORT_3D, TANGENT_2D,
        TANGENT_3D, TIME, UV,
    },
    module::{
        AsModule, DynAsModule, FieldDefinition, FunctionDefinition, InputDefinition,
        SpecializationData, StructDefinition,
    },
};

pub const CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Context", 1198218077110787867),
    public: true,
    fields: &[
        FieldDefinition {
            prop: POSITION_2D,
            public: true,
        },
        FieldDefinition {
            prop: POSITION_3D,
            public: true,
        },
        FieldDefinition {
            prop: TIME,
            public: true,
        },
        FieldDefinition {
            prop: DISTANCE,
            public: true,
        },
        FieldDefinition {
            prop: GRADIENT_2D,
            public: true,
        },
        FieldDefinition {
            prop: GRADIENT_3D,
            public: true,
        },
        FieldDefinition {
            prop: NORMAL,
            public: true,
        },
        FieldDefinition {
            prop: UV,
            public: true,
        },
        FieldDefinition {
            prop: TANGENT_2D,
            public: true,
        },
        FieldDefinition {
            prop: TANGENT_3D,
            public: true,
        },
        FieldDefinition {
            prop: COLOR,
            public: true,
        },
        FieldDefinition {
            prop: LIGHT,
            public: true,
        },
        FieldDefinition {
            prop: SUPPORT_2D,
            public: true,
        },
        FieldDefinition {
            prop: SUPPORT_3D,
            public: true,
        },
        FieldDefinition {
            prop: ERROR,
            public: true,
        },
    ],
};

pub struct Modify<T> {
    pub pre_modifiers: Vec<DynAsIR<T>>,
    pub field: DynAsModule<T>,
    pub post_modifiers: Vec<DynAsIR<T>>,
}

impl<T> Debug for Modify<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Modify")
            .field("pre_modifiers", &self.pre_modifiers)
            .field("field", &self.field)
            .field("post_modifiers", &self.post_modifiers)
            .finish()
    }
}

impl<T> Hash for Modify<T> {
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

impl<T> Modify<T>
where
    T: TypeSpec,
{
    pub fn translate(mut self, delta: crate::ast::expr::Expr<T>) -> Modify<T> {
        self.pre_modifiers.push(Box::new(Translate { delta }));
        self
    }

    pub fn elongate(mut self, dir: crate::ast::expr::Expr<T>, infinite: bool) -> Modify<T> {
        self.pre_modifiers
            .push(Box::new(Elongate { dir, infinite }));
        self
    }

    pub fn isosurface(mut self, dist: crate::ast::expr::Expr<T>) -> Modify<T> {
        self.post_modifiers.push(Box::new(Isosurface { dist }));
        self
    }

    pub fn manifold(mut self) -> Modify<T> {
        self.post_modifiers.push(Box::new(Manifold));
        self
    }

    pub fn gradient_normals(mut self) -> Modify<T>
    where
        T::NUMBER: IntoValue<T>,
    {
        self.post_modifiers.push(Box::new(GradientNormals));
        self
    }
}

impl<T> AsModule<T> for Modify<T>
where
    T: TypeSpec,
    T::NUMBER: IntoValue<T>,
    T::VECTOR2: IntoValue<T>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("modify")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition<T>> {
        let field_entry_point = self.field.entry_point();
        self.pre_modifiers
            .iter()
            .flat_map(|t| AsIR::functions(t, spec))
            .chain(self.field.functions(spec, &field_entry_point))
            .chain(
                self.post_modifiers
                    .iter()
                    .flat_map(|t| AsIR::functions(t, spec)),
            )
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
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

pub trait IntoModify<T>: 'static + Sized + AsModule<T>
where
    T: TypeSpec,
{
    fn modify(self) -> Modify<T> {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, U> IntoModify<U> for T
where
    T: 'static + Sized + AsModule<U>,
    U: TypeSpec,
{
}
