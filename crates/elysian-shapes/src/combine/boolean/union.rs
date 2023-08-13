use crate::{
    combine::{LEFT, OUT, RIGHT},
    shape::{DynShape, IntoShape},
};
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const UNION: FunctionIdentifier = FunctionIdentifier::new("union", 1894363406191409858);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Union;

impl IntoIterator for Union {
    type Item = DynShape;

    type IntoIter = std::iter::Once<DynShape>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.shape())
    }
}

impl Domains for Union {}

impl AsModule for Union {
    fn module_impl(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
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
