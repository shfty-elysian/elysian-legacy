mod blend;
mod boolean;

pub use blend::*;
pub use boolean::*;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::modify::CONTEXT_STRUCT;
use crate::ir::ast::CONTEXT;
use crate::ir::module::InputDefinition;
use crate::ir::{
    ast::{Block, Expr, IntoValue, LEFT, OUT, RIGHT},
    module::{FieldDefinition, FunctionDefinition, StructDefinition},
};

use crate::ir::{
    as_ir::{AsIR, HashIR},
    ast::{Identifier, VectorSpace},
    module::{AsModule, DynAsModule},
};

pub const COMBINE_CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
    id: Identifier::new("CombineContext", 416045102551943616),
    public: false,
    fields: &[
        FieldDefinition {
            prop: LEFT,
            public: false,
        },
        FieldDefinition {
            prop: RIGHT,
            public: false,
        },
        FieldDefinition {
            prop: OUT,
            public: false,
        },
    ],
};

pub struct Combine<T, const N: usize> {
    pub combinator: Vec<Box<dyn AsIR<T, N>>>,
    pub shapes: Vec<DynAsModule<T, N>>,
}

impl<T, const N: usize> Debug for Combine<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Combine")
            .field("combinator", &self.combinator)
            .field("shapes", &self.shapes)
            .finish()
    }
}

impl<T, const N: usize> Hash for Combine<T, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for combinator in &self.combinator {
            state.write_u64(combinator.hash_ir())
        }
        for shape in &self.shapes {
            state.write_u64(shape.hash_ir())
        }
    }
}

impl<T, const N: usize> AsModule<T, N> for Combine<T, N>
where
    T: VectorSpace<N>,
    T::NUMBER: IntoValue<T, N>,
    T::VECTOR2: IntoValue<T, N>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("combine")
    }

    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>> {
        let (shape_entry_points, shape_functions): (Vec<_>, Vec<_>) = self
            .shapes
            .iter()
            .map(|shape| {
                let entry_point = shape.entry_point();
                (entry_point.clone(), shape.functions(entry_point))
            })
            .unzip();

        let shape_functions: Vec<_> = shape_functions.into_iter().flatten().collect();

        let mut iter = shape_entry_points.iter();
        let base = iter.next().expect("Empty list").clone();

        let block = Block(vec![Expr::Read(
            Some(Box::new(iter.fold(
                Expr::Call {
                    function: base,
                    args: vec![CONTEXT.read()],
                },
                |acc, next| {
                    self.combinator.iter().fold(
                        Expr::Struct(
                            COMBINE_CONTEXT_STRUCT,
                            [
                                (LEFT, acc),
                                (
                                    RIGHT,
                                    Expr::Call {
                                        function: next.clone(),
                                        args: vec![CONTEXT.read()],
                                    },
                                ),
                            ]
                            .into(),
                        ),
                        |acc: Expr<T, N>, next| {
                            let Expr::Call{ function, args } = next.expression(acc) else  {
                            panic!("Combinator expression is not a CallResult")
                        };

                            Expr::Call { function, args }
                        },
                    )
                },
            ))),
            vec![OUT],
        )
        .output()]);

        self.combinator
            .iter()
            .flat_map(AsIR::functions)
            .chain(shape_functions)
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT,
                block,
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone(), COMBINE_CONTEXT_STRUCT.clone()]
    }
}
