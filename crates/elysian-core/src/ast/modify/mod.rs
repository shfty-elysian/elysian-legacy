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
        Block, Expr, Identifier, IntoValue, TypeSpec, VectorSpace, COLOR, CONTEXT, DISTANCE, ERROR,
        GRADIENT, LIGHT, POSITION, SUPPORT, TANGENT, TIME, UV,
    },
    module::{
        AsModule, DynAsModule, FieldDefinition, FunctionDefinition, InputDefinition,
        StructDefinition,
    },
};

pub const CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("Context", 1198218077110787867),
    public: true,
    fields: &[
        FieldDefinition {
            prop: POSITION,
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
            prop: GRADIENT,
            public: true,
        },
        FieldDefinition {
            prop: UV,
            public: true,
        },
        FieldDefinition {
            prop: TANGENT,
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
            prop: SUPPORT,
            public: true,
        },
        FieldDefinition {
            prop: ERROR,
            public: true,
        },
    ],
};

pub struct Modify<T, const N: usize> {
    pub pre_modifiers: Vec<DynAsIR<T, N>>,
    pub field: DynAsModule<T, N>,
    pub post_modifiers: Vec<DynAsIR<T, N>>,
}

impl<T, const N: usize> Debug for Modify<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Modify")
            .field("pre_modifiers", &self.pre_modifiers)
            .field("field", &self.field)
            .field("post_modifiers", &self.post_modifiers)
            .finish()
    }
}

impl<T, const N: usize> Hash for Modify<T, N> {
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

impl<T, const N: usize> Modify<T, N>
where
    T: VectorSpace<N>,
{
    pub fn translate(mut self, delta: crate::ast::expr::Expr<T>) -> Modify<T, N> {
        self.pre_modifiers.push(Box::new(Translate { delta }));
        self
    }

    pub fn elongate(mut self, dir: crate::ast::expr::Expr<T>, infinite: bool) -> Modify<T, N> {
        self.pre_modifiers
            .push(Box::new(Elongate { dir, infinite }));
        self
    }

    pub fn isosurface(mut self, dist: crate::ast::expr::Expr<T>) -> Modify<T, N> {
        self.post_modifiers.push(Box::new(Isosurface { dist }));
        self
    }

    pub fn manifold(mut self) -> Modify<T, N> {
        self.post_modifiers.push(Box::new(Manifold));
        self
    }
}

impl<T, const N: usize> AsModule<T, N> for Modify<T, N>
where
    T: VectorSpace<N>,
    T::NUMBER: IntoValue<T, N>,
    T::VECTOR2: IntoValue<T, N>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("modify")
    }

    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>> {
        let field_entry_point = self.field.entry_point();
        self.pre_modifiers
            .iter()
            .flat_map(AsIR::functions)
            .chain(self.field.functions(field_entry_point.clone()))
            .chain(self.post_modifiers.iter().flat_map(AsIR::functions))
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
                block: Block(vec![self
                    .post_modifiers
                    .iter()
                    .fold(
                        Expr::Call {
                            function: field_entry_point,
                            args: vec![self
                                .pre_modifiers
                                .iter()
                                .fold(CONTEXT.read(), |acc, next| next.expression(acc))],
                        },
                        |acc, next| next.expression(acc),
                    )
                    .output()]),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoModify<T, const N: usize>: 'static + Sized + AsModule<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn modify(self) -> Modify<T, N> {
        Modify {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, U, const N: usize> IntoModify<U, N> for T
where
    T: 'static + Sized + AsModule<U, N>,
    U: TypeSpec + VectorSpace<N>,
{
}
