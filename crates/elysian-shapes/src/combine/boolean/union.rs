use crate::combine::{Combinator, LEFT, OUT, RIGHT};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const UNION: FunctionIdentifier = FunctionIdentifier::new("union", 1894363406191409858);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union;

impl IntoIterator for Union {
    type Item = Box<dyn Combinator>;

    type IntoIter = std::iter::Once<Box<dyn Combinator>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(Box::new(self))
    }
}

impl Domains for Union {}

impl AsModule for Union {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        Module::new(
            self,
            spec,
            elysian_function! {
                fn UNION(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    if COMBINE_CONTEXT.LEFT.DISTANCE < COMBINE_CONTEXT.RIGHT.DISTANCE {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.LEFT;
                    }
                    else {
                        COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.RIGHT;
                    }

                    return COMBINE_CONTEXT;
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Combinator for Union {}
