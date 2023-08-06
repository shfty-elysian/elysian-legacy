use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::ir::module::DomainsDyn;
use crate::ir::{
    ast::{Block, Expr, Identifier, COMBINE_CONTEXT},
    module::{
        AsIR, DynAsIR, FieldDefinition, FunctionDefinition, FunctionIdentifier, HashIR,
        InputDefinition, PropertyIdentifier, SpecializationData, StructDefinition,
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
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("combine".into()).specialize(spec)
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prepared_shapes: Vec<_> = self
            .shapes
            .iter()
            .map(|t| {
                let (a, b, c) = t.prepare(spec);
                (t, a, b, c)
            })
            .collect();

        let mut iter = prepared_shapes.iter();
        let (base, _, base_entry, _) = iter.next().expect("Empty list").clone();

        let mut block = vec![];

        let combinators: Vec<_> = self
            .combinator
            .iter()
            .map(|t| {
                let (a, b, c) = t.prepare(spec);
                (t, a, b, c)
            })
            .collect();

        let combinator = combinators.iter().fold(
            elysian_stmt! { COMBINE_CONTEXT },
            |acc: Expr, (combinator, _, entry, _)| Expr::Call {
                function: entry.clone(),
                args: combinator.arguments(acc),
            },
        );

        iter.fold(
            base_entry.call(base.arguments(elysian_stmt! {CONTEXT})),
            |acc, (t, _, entry, _)| {
                let entry = entry.call(t.arguments(elysian_stmt! {CONTEXT}));

                block.push(elysian_stmt! {
                    let COMBINE_CONTEXT = COMBINE_CONTEXT {
                        LEFT: #acc,
                        RIGHT: #entry
                    }
                });

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

        combinators
            .iter()
            .flat_map(|(_, _, _, functions)| functions.clone())
            .chain(
                prepared_shapes
                    .iter()
                    .flat_map(|(_, _, _, functions)| functions.clone()),
            )
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
