use crate::{
    ast::combine::COMBINE_CONTEXT_STRUCT,
    ir::{
        as_ir::{AsIR, FilterSpec},
        ast::{
            Identifier, IntoBind, IntoBlock, IntoRead, COMBINE_CONTEXT, DISTANCE, LEFT, OUT, RIGHT,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boolean {
    Union,
    Intersection,
    Subtraction,
}

pub const UNION: Identifier = Identifier::new("union", 1894363406191409858);
pub const INTERSECTION: Identifier = Identifier::new("intersection", 18033822391797795038);
pub const SUBTRACTION: Identifier = Identifier::new("subtraction", 1414822549598552032);

impl FilterSpec for Boolean {}

impl AsIR for Boolean {
    fn functions_impl(&self, _: &SpecializationData) -> Vec<crate::ir::module::FunctionDefinition> {
        vec![FunctionDefinition {
            id: match self {
                Boolean::Union => UNION,
                Boolean::Intersection => INTERSECTION,
                Boolean::Subtraction => SUBTRACTION,
            },
            public: false,
            inputs: vec![InputDefinition {
                prop: COMBINE_CONTEXT,
                mutable: true,
            }],
            output: &COMBINE_CONTEXT_STRUCT,
            block: match self {
                Boolean::Union | Boolean::Intersection => {
                    [
                        [COMBINE_CONTEXT, OUT]
                            .bind([COMBINE_CONTEXT, LEFT].read())
                            .if_else(
                                match self {
                                    Boolean::Union => [COMBINE_CONTEXT, LEFT, DISTANCE]
                                        .read()
                                        .lt([COMBINE_CONTEXT, RIGHT, DISTANCE].read()),
                                    Boolean::Intersection => [COMBINE_CONTEXT, LEFT, DISTANCE]
                                        .read()
                                        .gt([COMBINE_CONTEXT, RIGHT, DISTANCE].read()),
                                    _ => unreachable!(),
                                },
                                Some([COMBINE_CONTEXT, OUT].bind([COMBINE_CONTEXT, RIGHT].read())),
                            ),
                        COMBINE_CONTEXT.read().output(),
                    ]
                    .block()
                }
                Boolean::Subtraction => [
                    [COMBINE_CONTEXT, OUT].bind([COMBINE_CONTEXT, RIGHT].read()),
                    [COMBINE_CONTEXT, OUT, DISTANCE].bind(-[COMBINE_CONTEXT, OUT, DISTANCE].read()),
                    [COMBINE_CONTEXT, OUT]
                        .bind([COMBINE_CONTEXT, LEFT].read())
                        .if_else(
                            [COMBINE_CONTEXT, LEFT, DISTANCE].read().gt([
                                COMBINE_CONTEXT,
                                OUT,
                                DISTANCE,
                            ]
                            .read()),
                            None,
                        ),
                    COMBINE_CONTEXT.read().output(),
                ]
                .block(),
            },
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: crate::ir::ast::Expr,
    ) -> crate::ir::ast::Expr {
        match self {
            Boolean::Union => crate::ir::ast::Expr::Call {
                function: UNION,
                args: vec![input],
            },
            Boolean::Intersection => crate::ir::ast::Expr::Call {
                function: INTERSECTION,
                args: vec![input],
            },
            Boolean::Subtraction => crate::ir::ast::Expr::Call {
                function: SUBTRACTION,
                args: vec![input],
            },
        }
    }
}
