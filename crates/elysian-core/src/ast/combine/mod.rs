mod blend;
mod boolean;

pub use blend::*;
pub use boolean::*;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ast::modify::CONTEXT_STRUCT;
use crate::ir::ast::{TypeSpec, CONTEXT};
use crate::ir::module::InputDefinition;
use crate::ir::{
    ast::{Block, Expr, IntoValue, LEFT, OUT, RIGHT},
    module::{FieldDefinition, FunctionDefinition, StructDefinition},
};

use crate::ir::{
    as_ir::{AsIR, HashIR},
    ast::Identifier,
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

pub struct Combine<T> {
    pub combinator: Vec<Box<dyn AsIR<T>>>,
    pub shapes: Vec<DynAsModule<T>>,
}

impl<T> Debug for Combine<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Combine")
            .field("combinator", &self.combinator)
            .field("shapes", &self.shapes)
            .finish()
    }
}

impl<T> Hash for Combine<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for combinator in &self.combinator {
            state.write_u64(combinator.hash_ir())
        }
        for shape in &self.shapes {
            state.write_u64(shape.hash_ir())
        }
    }
}

impl<T> AsModule<T> for Combine<T>
where
    T: TypeSpec,
    T::NUMBER: IntoValue<T>,
    T::VECTOR2: IntoValue<T>,
{
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("combine")
    }

    fn functions(&self, entry_point: &Identifier) -> Vec<FunctionDefinition<T>> {
        let (shape_entry_points, shape_functions): (Vec<_>, Vec<_>) = self
            .shapes
            .iter()
            .map(|shape| {
                let entry_point = shape.entry_point();
                (entry_point.clone(), shape.functions(&entry_point))
            })
            .unzip();

        let shape_functions: Vec<_> = shape_functions.into_iter().flatten().collect();

        let mut iter = shape_entry_points.iter();
        let base = iter.next().expect("Empty list").clone();

        let block = Block(vec![iter
            .fold(base.call(CONTEXT.read()), |acc, next| {
                self.combinator.iter().fold(
                    COMBINE_CONTEXT_STRUCT
                        .construct([(LEFT, acc), (RIGHT, next.call(CONTEXT.read()))]),
                    |acc: Expr<T>, next| {
                        let Expr::Call{ function, args } = next.expression(acc) else  {
                            panic!("Combinator expression is not a CallResult")
                        };

                        Expr::Call { function, args }
                    },
                )
            })
            .read([OUT])
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
