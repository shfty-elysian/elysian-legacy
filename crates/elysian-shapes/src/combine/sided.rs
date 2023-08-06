use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        ast::{COMBINE_CONTEXT, DISTANCE},
        module::{
            AsIR, Domains, FunctionDefinition, FunctionIdentifier, PropertyIdentifier,
            SpecializationData,
        },
    },
};
use elysian_decl_macros::elysian_function;

pub const SIDED: FunctionIdentifier = FunctionIdentifier::new("sided", 19756903452063788266);

// Pick a base context from either the left or right side
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sided {
    pub flip: bool,
}

impl Domains for Sided {}

impl AsIR for Sided {
    fn entry_point(&self, spec: &SpecializationData) -> FunctionIdentifier {
        SIDED.specialize(spec)
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let side = if self.flip { RIGHT } else { LEFT };

        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.side;
                return COMBINE_CONTEXT
            }
        }]
    }
}

// Pick the given property from the left if inside, or right if outside
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SidedProp {
    pub prop: PropertyIdentifier,
    pub flip: bool,
}

impl Domains for SidedProp {}

impl AsIR for SidedProp {
    fn entry_point(&self, _: &SpecializationData) -> FunctionIdentifier {
        (*SIDED).concat(&self.prop).into()
    }

    fn functions(
        &self,
        _: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let prop = &self.prop;

        let (left, right) = if self.flip {
            (RIGHT, LEFT)
        } else {
            (LEFT, RIGHT)
        };

        vec![elysian_function! {
            fn entry_point(mut COMBINE_CONTEXT) -> COMBINE_CONTEXT {
                if COMBINE_CONTEXT.OUT.DISTANCE > 0.0 {
                    COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.left.prop;
                }
                else {
                    COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.right.prop;
                }

                return COMBINE_CONTEXT;
            }
        }]
    }
}
