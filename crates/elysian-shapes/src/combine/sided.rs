use crate::combine::{Combinator, LEFT, OUT, RIGHT};
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_decl_macros::elysian_function;
use elysian_ir::{
    ast::{COMBINE_CONTEXT, DISTANCE},
    module::{AsModule, Domains, FunctionIdentifier, Module, SpecializationData},
};

pub const SIDED: FunctionIdentifier = FunctionIdentifier::new("sided", 19756903452063788266);

// Pick a base context from either the left or right side
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sided {
    flip: bool,
}

impl Sided {
    pub fn left() -> Self {
        Sided { flip: false }
    }

    pub fn right() -> Self {
        Sided { flip: true }
    }
}

impl Domains for Sided {}

impl AsModule for Sided {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let side = if self.flip { RIGHT } else { LEFT };

        Module::new(
            self,
            spec,
            elysian_function! {
                fn SIDED(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.side;
                    return COMBINE_CONTEXT
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Combinator for Sided {}

// Pick the given property from the left if inside, or right if outside
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SidedProp {
    prop: PropertyIdentifier,
    flip: bool,
}

impl SidedProp {
    pub fn new(prop: impl Into<PropertyIdentifier>, flip: bool) -> Self {
        SidedProp {
            prop: prop.into(),
            flip,
        }
    }
}

impl Domains for SidedProp {}

impl AsModule for SidedProp {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let sided_prop: FunctionIdentifier = (*SIDED).concat(&self.prop).into();

        let prop = &self.prop;

        let (left, right) = if self.flip {
            (RIGHT, LEFT)
        } else {
            (LEFT, RIGHT)
        };

        Module::new(
            self,
            spec,
            elysian_function! {
                fn sided_prop(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                    if COMBINE_CONTEXT.OUT.DISTANCE > 0.0 {
                        COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.left.prop;
                    }
                    else {
                        COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.right.prop;
                    }

                    return COMBINE_CONTEXT;
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Combinator for SidedProp {}
