use crate::{
    ast::combine::COMBINE_CONTEXT_STRUCT,
    ir::{
        as_ir::AsIR,
        ast::{
            Identifier, IntoBlock, IntoRead, IntoWrite, TypeSpec, COMBINE_CONTEXT,
            DISTANCE, LEFT, OUT, RIGHT,
        },
        module::{FunctionDefinition, InputDefinition},
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

impl<T> AsIR<T> for Boolean
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T>> {
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
                            .write([COMBINE_CONTEXT, LEFT].read())
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
                                Some([COMBINE_CONTEXT, OUT].write([COMBINE_CONTEXT, RIGHT].read())),
                            ),
                        COMBINE_CONTEXT.read().output(),
                    ]
                    .block()
                }
                Boolean::Subtraction => [
                    [COMBINE_CONTEXT, OUT].write([COMBINE_CONTEXT, RIGHT].read()),
                    [COMBINE_CONTEXT, OUT, DISTANCE]
                        .write(-[COMBINE_CONTEXT, OUT, DISTANCE].read()),
                    [COMBINE_CONTEXT, OUT]
                        .write([COMBINE_CONTEXT, LEFT].read())
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

    fn expression(&self, input: crate::ir::ast::Expr<T>) -> crate::ir::ast::Expr<T> {
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
