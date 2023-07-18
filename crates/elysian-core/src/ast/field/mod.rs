mod capsule;
mod circle;
mod line;
mod point;
mod ring;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub use capsule::*;
pub use circle::*;
pub use line::*;
pub use point::*;
pub use ring::*;

use crate::ir::{
    as_ir::{AsIR, DynAsIR},
    ast::{
        Block, Identifier, IntoRead, IntoValue, IntoWrite, Property, Stmt, TypeSpec, VectorSpace,
        COLOR, CONTEXT, DISTANCE, ERROR, GRADIENT, LIGHT, POSITION, SUPPORT, TANGENT, TIME, UV,
    },
    module::{AsModule, FieldDefinition, FunctionDefinition, InputDefinition, StructDefinition},
};

use super::{
    post_modifier::{isosurface::Isosurface, manifold::Manifold},
    pre_modifier::{elongate::Elongate, translate::Translate},
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

pub struct Field<T, const N: usize> {
    pub pre_modifiers: Vec<DynAsIR<T, N>>,
    pub field: DynAsIR<T, N>,
    pub post_modifiers: Vec<DynAsIR<T, N>>,
}

impl<T, const N: usize> Debug for Field<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("pre_modifiers", &self.pre_modifiers)
            .field("field", &self.field)
            .field("post_modifiers", &self.post_modifiers)
            .finish()
    }
}

impl<T, const N: usize> Hash for Field<T, N> {
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

impl<T, const N: usize> Field<T, N>
where
    T: VectorSpace<N>,
{
    pub fn translate(mut self, delta: crate::ast::expr::Expr<T>) -> Field<T, N> {
        self.pre_modifiers.push(Box::new(Translate { delta }));
        self
    }

    pub fn elongate(mut self, dir: crate::ast::expr::Expr<T>, infinite: bool) -> Field<T, N> {
        self.pre_modifiers
            .push(Box::new(Elongate { dir, infinite }));
        self
    }

    pub fn isosurface(mut self, dist: crate::ast::expr::Expr<T>) -> Field<T, N> {
        self.post_modifiers.push(Box::new(Isosurface { dist }));
        self
    }

    pub fn manifold(mut self) -> Field<T, N> {
        self.post_modifiers.push(Box::new(Manifold));
        self
    }
}

impl<T, const N: usize> AsModule<T, N> for Field<T, N>
where
    T: VectorSpace<N>,
    T::NUMBER: IntoValue<T, N>,
    T::VECTOR2: IntoValue<T, N>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("field")
    }

    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>> {
        self.pre_modifiers
            .iter()
            .flat_map(AsIR::functions)
            .chain(self.field.functions())
            .chain(self.post_modifiers.iter().flat_map(AsIR::functions))
            .chain([FunctionDefinition {
                id: entry_point,
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
                block: Block(
                    self.pre_modifiers
                        .iter()
                        .map(|modifier| Stmt::Write {
                            path: vec![CONTEXT],
                            expr: modifier.expression(CONTEXT.read()),
                        })
                        .chain([Stmt::Write {
                            path: vec![CONTEXT],
                            expr: self.field.expression(CONTEXT.read()),
                        }])
                        .chain(self.post_modifiers.iter().map(|modifier| Stmt::Write {
                            path: vec![CONTEXT],
                            expr: modifier.expression(CONTEXT.read()),
                        }))
                        .chain(std::iter::once([CONTEXT].read().output()))
                        .collect(),
                ),
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoField<T, const N: usize>: 'static + Sized + AsIR<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn field(self) -> Field<T, N> {
        Field {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, U, const N: usize> IntoField<U, N> for T
where
    T: 'static + Sized + AsIR<U, N>,
    U: TypeSpec + VectorSpace<N>,
{
}
