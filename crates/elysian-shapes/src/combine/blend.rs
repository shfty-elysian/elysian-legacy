use std::fmt::Debug;

use elysian_core::{
    ast::{
        combine::{LEFT, OUT, RIGHT},
        expr::Expr,
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoLiteral, IntoRead, IntoWrite, COMBINE_CONTEXT, DISTANCE, NUM,
        },
        module::{
            FunctionDefinition, InputDefinition, NumericType, SpecializationData, Type, 
        },
    }, property,
};

pub const SMOOTH_UNION: Identifier = Identifier::new("smooth_union", 1894363406191409858);
pub const SMOOTH_INTERSECTION: Identifier =
    Identifier::new("smooth_intersection", 18033822391797795038);
pub const SMOOTH_SUBTRACTION: Identifier =
    Identifier::new("smooth_subtraction", 1414822549598552032);

pub const K: Identifier = Identifier::new("k", 12632115441234896764);
property!(K, K_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub enum Blend {
    SmoothUnion { prop: Identifier, k: Expr },
    SmoothIntersection { prop: Identifier, k: Expr },
    SmoothSubtraction { prop: Identifier, k: Expr },
}

impl std::hash::Hash for Blend {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Blend::SmoothUnion { prop, k } => {
                prop.hash(state);
                k.hash(state);
            }
            Blend::SmoothIntersection { prop, k } => {
                prop.hash(state);
                k.hash(state);
            }
            Blend::SmoothSubtraction { prop, k } => {
                prop.hash(state);
                k.hash(state);
            }
        }
    }
}

impl Domains for Blend {}

impl AsIR for Blend {
    fn functions_impl(
        &self,
        _: &SpecializationData,
    ) -> Vec<elysian_core::ir::module::FunctionDefinition> {
        vec![FunctionDefinition {
            id: match self {
                Blend::SmoothUnion { prop, .. } => SMOOTH_UNION.concat(prop),
                Blend::SmoothIntersection { prop, .. } => SMOOTH_INTERSECTION.concat(prop),
                Blend::SmoothSubtraction { prop, .. } => SMOOTH_SUBTRACTION.concat(prop),
            },
            public: false,
            inputs: vec![
                InputDefinition {
                    id: K,
                    mutable: false,
                },
                InputDefinition {
                    id: COMBINE_CONTEXT,
                    mutable: true,
                },
            ],
            output: COMBINE_CONTEXT,
            block: match self {
                Blend::SmoothUnion { prop, .. } => {
                    let mut block = vec![
                        NUM.bind(
                            (0.5.literal()
                                + 0.5.literal()
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        - [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(0.0.literal())
                            .min(1.0.literal()),
                        ),
                        [COMBINE_CONTEXT, OUT, prop.clone()].write(
                            [COMBINE_CONTEXT, RIGHT, prop.clone()]
                                .read()
                                .mix([COMBINE_CONTEXT, LEFT, prop.clone()].read(), NUM.read()),
                        ),
                    ];

                    if *prop == DISTANCE {
                        block.push([COMBINE_CONTEXT, OUT, DISTANCE].write(
                            [COMBINE_CONTEXT, OUT, DISTANCE].read()
                                - K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { prop, .. } => {
                    let property: Identifier = prop.clone().into();

                    let mut block = vec![
                        NUM.bind(
                            (0.5.literal()
                                - 0.5.literal()
                                    * ([RIGHT, DISTANCE].read() - [LEFT, DISTANCE].read())
                                    / K.read())
                            .max(0.0.literal())
                            .min(1.0.literal()),
                        ),
                        property.clone().bind(
                            [RIGHT, property.clone()]
                                .read()
                                .mix([LEFT, property.clone()].read(), NUM.read()),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push(DISTANCE.bind(
                            DISTANCE.read() + K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push([OUT, property.clone()].write([OUT, property.clone()].read()));

                    block.into_iter().collect()
                }
                Blend::SmoothSubtraction { prop, .. } => {
                    let property: Identifier = prop.clone().into();

                    let mut block = vec![
                        NUM.bind(
                            (0.5.literal()
                                - 0.5.literal()
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        + [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(0.0.literal())
                            .min(1.0.literal()),
                        ),
                        [COMBINE_CONTEXT, OUT, property.clone()].write(
                            [COMBINE_CONTEXT, LEFT, property.clone()].read().mix(
                                -[COMBINE_CONTEXT, RIGHT, property.clone()].read(),
                                NUM.read(),
                            ),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push([COMBINE_CONTEXT, OUT, DISTANCE].write(
                            [COMBINE_CONTEXT, OUT, DISTANCE].read()
                                + K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
            },
        }]
    }

    fn expression_impl(
        &self,
        _: &SpecializationData,
        input: elysian_core::ir::ast::Expr,
    ) -> elysian_core::ir::ast::Expr {
        match self {
            Blend::SmoothUnion { prop, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_UNION.concat(prop),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothIntersection { prop, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_INTERSECTION.concat(prop),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothSubtraction { prop, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_SUBTRACTION.concat(prop),
                args: vec![k.clone().into(), input],
            },
        }
    }
}
