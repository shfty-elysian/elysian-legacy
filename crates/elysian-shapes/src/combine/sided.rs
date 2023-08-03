use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Block, COMBINE_CONTEXT, DISTANCE},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, PropertyIdentifier,
            SpecializationData,
        },
    },
};
use elysian_proc_macros::{elysian_block, elysian_stmt};

pub const SIDED: FunctionIdentifier = FunctionIdentifier::new("sided", 19756903452063788266);

// Pick a base context from either the left or right side
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sided {
    pub flip: bool,
}

impl Domains for Sided {}

impl AsIR for Sided {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        let mut block = Block::default();

        let side = if self.flip { RIGHT } else { LEFT };

        block.extend(elysian_block! {
            COMBINE_CONTEXT.OUT = COMBINE_CONTEXT.side;
        });

        block.push(elysian_stmt! {
            return COMBINE_CONTEXT
        });

        vec![FunctionDefinition {
            id: SIDED,
            public: false,
            inputs: vec![InputDefinition {
                id: COMBINE_CONTEXT.into(),
                mutable: true,
            }],
            output: COMBINE_CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        elysian_core::ir::ast::Expr::Call {
            function: SIDED,
            args: vec![input],
        }
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
    fn functions_impl(&self, _: &SpecializationData) -> Vec<FunctionDefinition> {
        let mut block = Block::default();

        let prop = &self.prop;

        let sided = FunctionIdentifier((*SIDED).concat(prop));

        let (left, right) = if self.flip {
            (RIGHT, LEFT)
        } else {
            (LEFT, RIGHT)
        };

        block.extend(elysian_block! {
            if COMBINE_CONTEXT.OUT.DISTANCE > 0.0 {
                COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.left.prop;
            }
            else {
                COMBINE_CONTEXT.OUT.prop = COMBINE_CONTEXT.right.prop;
            }
        });

        block.push(elysian_stmt! {
            return COMBINE_CONTEXT
        });

        vec![FunctionDefinition {
            id: sided,
            public: false,
            inputs: vec![InputDefinition {
                id: COMBINE_CONTEXT.into(),
                mutable: true,
            }],
            output: COMBINE_CONTEXT.into(),
            block,
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        elysian_core::ir::ast::Expr::Call {
            function: (*SIDED).concat(&self.prop).into(),
            args: vec![input],
        }
    }
}
