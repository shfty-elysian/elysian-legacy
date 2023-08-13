mod blend;
mod boolean;
mod builder;
mod displace;
mod overlay;
mod sided;

pub use blend::*;
pub use boolean::*;
pub use builder::*;
pub use displace::*;
use elysian_core::identifier::Identifier;
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::{AsModule, Module};
pub use overlay::*;
pub use overlay::*;
pub use sided::*;

use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_ir::{
    ast::{Block, Expr, COMBINE_CONTEXT},
    module::{
        DomainsDyn, ErasedHash, FieldDefinition, FunctionDefinition, FunctionIdentifier,
        InputDefinition, SpecializationData, StructDefinition, StructIdentifier, Type, CONTEXT,
    },
    property,
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use crate::shape::{DynShape, IntoShape, Shape};

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

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Combine {
    pub combinator: Vec<Box<dyn Combinator>>,
    pub shapes: Vec<DynShape>,
}

impl<T> From<T> for Combine
where
    T: IntoIterator<Item = Box<dyn Combinator>>,
{
    fn from(value: T) -> Self {
        value.into_iter().collect()
    }
}

impl FromIterator<Box<dyn Combinator>> for Combine {
    fn from_iter<T: IntoIterator<Item = Box<dyn Combinator>>>(iter: T) -> Self {
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
            state.write_u64(combinator.erased_hash())
        }
        for shape in &self.shapes {
            state.write_u64(shape.erased_hash())
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

impl AsModule for Combine {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        assert!(self.shapes.len() > 0, "Empty Combine");

        let prepared_shapes: Vec<_> = self.shapes.iter().map(|t| t.module(spec)).collect();

        let mut iter = prepared_shapes.iter();
        let base_module = iter.next().expect("Empty list").clone();

        let mut block = vec![];

        let combinators: Vec<_> = self.combinator.iter().map(|t| t.module(spec)).collect();

        let combinator = combinators
            .iter()
            .fold(elysian_stmt! { COMBINE_CONTEXT }, |acc: Expr, module| {
                module.call(acc)
            });

        if prepared_shapes.len() == 1 {
            let base_call = base_module.call(elysian_stmt! {CONTEXT});
            block.push(elysian_stmt! {
                return #base_call
            });
        } else {
            iter.fold(base_module.call(elysian_stmt! {CONTEXT}), |acc, entry| {
                let entry = entry.call(elysian_stmt! {CONTEXT});

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
            });

            block.push(elysian_stmt! {
                return OUT
            });
        }

        let block = Block(block);

        let mut module = combinators
            .into_iter()
            .fold(Module::default(), |acc, next| acc.concat(next))
            .concat(
                prepared_shapes
                    .into_iter()
                    .fold(Module::default(), |acc, next| acc.concat(next)),
            )
            .concat(Module::new(
                self,
                spec,
                FunctionDefinition {
                    id: FunctionIdentifier::new_dynamic("combine".into()),
                    public: true,
                    inputs: vec![InputDefinition {
                        id: PropertyIdentifier(CONTEXT),
                        mutable: false,
                    }],
                    output: PropertyIdentifier(CONTEXT),
                    block,
                },
            ));

        module
            .struct_definitions
            .push(COMBINE_CONTEXT_STRUCT.clone());

        module
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Combine {}

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Combinator: Debug + AsModule + ErasedHash + DomainsDyn {}

pub trait IntoCombinator: 'static + Sized + Combinator {
    fn pre_modifier(self) -> Box<dyn Combinator> {
        Box::new(self)
    }
}

impl<T> IntoCombinator for T where T: 'static + Combinator {}
