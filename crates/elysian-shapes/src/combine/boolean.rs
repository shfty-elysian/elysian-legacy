use elysian_core::{
    ast::combine::{LEFT, OUT, RIGHT},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{Identifier, IntoBlock, COMBINE_CONTEXT_PROP, DISTANCE},
        module::{FunctionDefinition, InputDefinition, IntoRead, IntoWrite, SpecializationData},
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

impl Domains for Boolean {}

impl AsIR for Boolean {
    fn functions_impl(
        &self,
        _: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        vec![FunctionDefinition {
            id: match self {
                Boolean::Union => UNION,
                Boolean::Intersection => INTERSECTION,
                Boolean::Subtraction => SUBTRACTION,
            },
            public: false,
            inputs: vec![InputDefinition {
                id: COMBINE_CONTEXT_PROP,
                mutable: true,
            }],
            output: COMBINE_CONTEXT_PROP,
            block: match self {
                Boolean::Union | Boolean::Intersection => [
                    [COMBINE_CONTEXT_PROP, OUT]
                        .write([COMBINE_CONTEXT_PROP, LEFT].read())
                        .if_else(
                            match self {
                                Boolean::Union => [COMBINE_CONTEXT_PROP, LEFT, DISTANCE]
                                    .read()
                                    .lt([COMBINE_CONTEXT_PROP, RIGHT, DISTANCE].read()),
                                Boolean::Intersection => [COMBINE_CONTEXT_PROP, LEFT, DISTANCE]
                                    .read()
                                    .gt([COMBINE_CONTEXT_PROP, RIGHT, DISTANCE].read()),
                                _ => unreachable!(),
                            },
                            Some(
                                [COMBINE_CONTEXT_PROP, OUT]
                                    .write([COMBINE_CONTEXT_PROP, RIGHT].read()),
                            ),
                        ),
                    COMBINE_CONTEXT_PROP.read().output(),
                ]
                .block(),
                Boolean::Subtraction => [
                    [COMBINE_CONTEXT_PROP, OUT].write([COMBINE_CONTEXT_PROP, RIGHT].read()),
                    [COMBINE_CONTEXT_PROP, OUT, DISTANCE]
                        .write(-[COMBINE_CONTEXT_PROP, OUT, DISTANCE].read()),
                    [COMBINE_CONTEXT_PROP, OUT]
                        .write([COMBINE_CONTEXT_PROP, LEFT].read())
                        .if_else(
                            [COMBINE_CONTEXT_PROP, LEFT, DISTANCE].read().gt([
                                COMBINE_CONTEXT_PROP,
                                OUT,
                                DISTANCE,
                            ]
                            .read()),
                            None,
                        ),
                    COMBINE_CONTEXT_PROP.read().output(),
                ]
                .block(),
            },
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        match self {
            Boolean::Union => elysian_core::ir::ast::Expr::Call {
                function: UNION,
                args: vec![input],
            },
            Boolean::Intersection => elysian_core::ir::ast::Expr::Call {
                function: INTERSECTION,
                args: vec![input],
            },
            Boolean::Subtraction => elysian_core::ir::ast::Expr::Call {
                function: SUBTRACTION,
                args: vec![input],
            },
        }
    }
}
