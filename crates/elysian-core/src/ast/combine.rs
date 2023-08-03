use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ir::module::DomainsDyn;
use crate::ir::{
    ast::{Block, Expr, Identifier, COMBINE_CONTEXT},
    module::{
        AsIR, DynAsIR, FieldDefinition, FunctionDefinition, FunctionIdentifier, HashIR,
        InputDefinition, IntoRead, PropertyIdentifier, SpecializationData, StructDefinition,
        StructIdentifier, Type, CONTEXT,
    },
};
use crate::property;
use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const LEFT: Identifier = Identifier::new("left", 635254731934742132);
property!(LEFT, LEFT_PROP_DEF, Type::Struct(StructIdentifier(CONTEXT)));

pub const RIGHT: Identifier = Identifier::new("right", 5251097991491214179);
property!(
    RIGHT,
    RIGHT_PROP_DEF,
    Type::Struct(StructIdentifier(CONTEXT))
);

pub const OUT: Identifier = Identifier::new("out", 1470763158891875334);
property!(OUT, OUT_PROP_DEF, Type::Struct(StructIdentifier(CONTEXT)));

pub const COMBINE_CONTEXT_STRUCT_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition {
        id: PropertyIdentifier(LEFT),
        public: false,
    },
    FieldDefinition {
        id: PropertyIdentifier(RIGHT),
        public: false,
    },
    FieldDefinition {
        id: PropertyIdentifier(OUT),
        public: false,
    },
];

pub const COMBINE_CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
    id: StructIdentifier(COMBINE_CONTEXT),
    public: false,
    fields: Cow::Borrowed(COMBINE_CONTEXT_STRUCT_FIELDS),
};

pub struct Combine {
    pub combinator: Vec<DynAsIR>,
    pub shapes: Vec<DynAsIR>,
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

impl DomainsDyn for Combine {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.combinator
            .iter()
            .flat_map(|t| t.domains_dyn())
            .chain(self.shapes.iter().flat_map(|t| t.domains_dyn()))
            .collect()
    }
}

impl AsIR for Combine {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("combine")
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (shape_entry_points, shape_functions): (Vec<_>, Vec<_>) = self
            .shapes
            .iter()
            .map(|shape| {
                let entry_point = shape.entry_point(spec);
                (
                    entry_point.clone(),
                    shape.functions(spec, &entry_point),
                )
            })
            .unzip();

        let shape_functions: Vec<_> = shape_functions.into_iter().flatten().collect();

        let mut iter = shape_entry_points.iter();
        let base = iter.next().expect("Empty list").clone();

        let mut block = vec![];

        iter.fold(
            base.call(PropertyIdentifier(CONTEXT).read()),
            |acc, next| {
                let next = (**next).clone();

                block.push(elysian_stmt! {
                    let COMBINE_CONTEXT = COMBINE_CONTEXT {
                        LEFT: #acc,
                        RIGHT: #next(CONTEXT)
                    }
                });

                let combinator = self.combinator.iter().fold(
                    elysian_stmt! { COMBINE_CONTEXT },
                    |acc: Expr, next| {
                        let Expr::Call{ function, args } = next.expression(spec, acc) else  {
                            panic!("Combinator expression is not a Call")
                        };

                        Expr::Call { function, args }
                    },
                );

                block.extend(elysian_block! {
                    let COMBINE_CONTEXT = #combinator;
                    let OUT = COMBINE_CONTEXT.OUT;
                });

                elysian_stmt! { OUT }
            },
        );

        block.push(elysian_stmt! {
            return OUT
        });

        let block = Block(block);

        self.combinator
            .iter()
            .flat_map(|t| t.functions_internal(spec))
            .chain(shape_functions)
            .chain([FunctionDefinition {
                id: entry_point.clone(),
                public: true,
                inputs: vec![InputDefinition {
                    id: PropertyIdentifier(CONTEXT),
                    mutable: false,
                }],
                output: PropertyIdentifier(CONTEXT),
                block,
            }])
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        vec![COMBINE_CONTEXT_STRUCT.clone()]
    }
}

pub trait IntoCombine {
    fn combine<U>(self, combinator: U) -> Combine
    where
        U: IntoIterator<Item = DynAsIR>;
}

impl<T> IntoCombine for T
where
    T: IntoIterator<Item = DynAsIR>,
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
