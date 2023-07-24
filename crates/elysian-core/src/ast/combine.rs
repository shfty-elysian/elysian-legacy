use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ir::ast::{IntoRead, COMBINE_CONTEXT};
use crate::{
    ast::modify::CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, DynAsIR, HashIR},
        ast::{Block, Expr, Identifier, CONTEXT, LEFT, OUT, RIGHT},
        module::{
            AsModule, DynAsModule, FieldDefinition, FunctionDefinition, InputDefinition,
            SpecializationData, StructDefinition,
        },
    },
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

pub struct Combine {
    pub combinator: Vec<Box<dyn AsIR>>,
    pub shapes: Vec<DynAsModule>,
}

impl Debug for Combine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Combine")
            .field("combinator", &self.combinator)
            .field("shapes", &self.shapes)
            .finish()
    }
}

impl Hash for Combine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for combinator in &self.combinator {
            state.write_u64(combinator.hash_ir())
        }
        for shape in &self.shapes {
            state.write_u64(shape.hash_ir())
        }
    }
}

impl AsModule for Combine {
    fn entry_point(&self) -> Identifier {
        Identifier::new_dynamic("combine")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        let (shape_entry_points, shape_functions): (Vec<_>, Vec<_>) = self
            .shapes
            .iter()
            .map(|shape| {
                let entry_point = shape.entry_point();
                (entry_point.clone(), shape.functions(spec, &entry_point))
            })
            .unzip();

        let shape_functions: Vec<_> = shape_functions.into_iter().flatten().collect();

        let mut iter = shape_entry_points.iter();
        let base = iter.next().expect("Empty list").clone();

        let mut block = vec![];

        iter.fold(base.call(CONTEXT.read()), |acc, next| {
            block.push(COMBINE_CONTEXT.bind(
                COMBINE_CONTEXT_STRUCT.construct([(LEFT, acc), (RIGHT, next.call(CONTEXT.read()))]),
            ));
            block.push(COMBINE_CONTEXT.bind(self.combinator.iter().fold(
                COMBINE_CONTEXT.read(),
                |acc: Expr, next| {
                    let Expr::Call{ function, args } = next.expression(spec, acc) else  {
                            panic!("Combinator expression is not a Call")
                        };

                    Expr::Call { function, args }
                },
            )));
            block.push(OUT.bind([COMBINE_CONTEXT, OUT].read()));
            OUT.read()
        });

        block.push(OUT.read().output());

        let block = Block(block);

        self.combinator
            .iter()
            .flat_map(|t| t.functions(spec))
            .chain(shape_functions)
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    prop: CONTEXT,
                    mutable: false,
                }],
                output: CONTEXT_STRUCT.clone(),
                block,
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![CONTEXT_STRUCT.clone(), COMBINE_CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoCombine {
    fn combine<U>(self, combinator: U) -> Combine
    where
        U: IntoIterator<Item = DynAsIR>;
}

impl<T> IntoCombine for T
where
    T: IntoIterator<Item = DynAsModule>,
{
    fn combine<V>(self, combinator: V) -> Combine
    where
        V: IntoIterator<Item = DynAsIR>,
    {
        let shapes: Vec<_> = self.into_iter().collect();
        assert!(shapes.len() >= 2, "Combine must have at least two shapes");
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes,
        }
    }
}
