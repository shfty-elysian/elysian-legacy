mod blend;
mod boolean;
mod displace;
mod overlay;
mod sided;

pub use blend::*;
pub use boolean::*;
pub use displace::*;
use elysian_core::identifier::Identifier;
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::Prepare;
pub use overlay::*;
pub use overlay::*;
pub use sided::*;

use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_ir::{
    ast::{Block, Expr, COMBINE_CONTEXT},
    module::{
        AsIR, DomainsDyn, FieldDefinition, FunctionDefinition, FunctionIdentifier, HashIR,
        InputDefinition, SpecializationData, StructDefinition, StructIdentifier, Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::shape::{DynShape, IntoShape};

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

#[derive(Debug)]
pub struct Combinator(Vec<DynShape>);

impl Combinator {
    pub fn build() -> Self {
        Combinator(Default::default())
    }

    pub fn push(mut self, combinator: impl IntoShape) -> Self {
        self.0.push(combinator.shape());
        self
    }

    pub fn combine(self) -> Combine {
        Combine {
            combinator: self.into_iter().collect(),
            shapes: Default::default(),
        }
    }
}

impl IntoIterator for Combinator {
    type Item = DynShape;

    type IntoIter = std::vec::IntoIter<DynShape>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Combine {
    pub combinator: Vec<DynShape>,
    pub shapes: Vec<DynShape>,
}

impl<T> From<T> for Combine
where
    T: IntoIterator<Item = DynShape>,
{
    fn from(value: T) -> Self {
        value.into_iter().collect()
    }
}

impl FromIterator<DynShape> for Combine {
    fn from_iter<T: IntoIterator<Item = DynShape>>(iter: T) -> Self {
        Combine {
            combinator: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl Combine {
    pub fn push(mut self, shape: impl IntoShape + 'static) -> Self {
        self.shapes.push(shape.shape());
        self
    }
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
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("combine".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        assert!(self.shapes.len() > 0, "Empty Combine");

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

        if prepared_shapes.len() == 1 {
            let base_call = base_entry.call(base.arguments(elysian_stmt! {CONTEXT}));
            block.push(elysian_stmt! {
                return #base_call
            });
        } else {
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
        }

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
