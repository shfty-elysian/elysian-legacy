use std::fmt::Debug;

use elysian_core::{
    ast::{
        combine::{LEFT, OUT, RIGHT},
        expr::Expr,
    },
    ir::{
        as_ir::{AsIR, Domains},
        ast::{IntoLiteral, COMBINE_CONTEXT_PROP, DISTANCE, NUM},
        module::{
            FunctionDefinition, FunctionIdentifier, InputDefinition, IntoRead, IntoWrite,
            NumericType, PropertyIdentifier, SpecializationData, Type,
        },
    },
    property,
};

pub const SMOOTH_UNION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_union", 1894363406191409858);
pub const SMOOTH_INTERSECTION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_intersection", 18033822391797795038);
pub const SMOOTH_SUBTRACTION: FunctionIdentifier =
    FunctionIdentifier::new("smooth_subtraction", 1414822549598552032);

pub const K: PropertyIdentifier = PropertyIdentifier::new("k", 12632115441234896764);
property!(K, K_PROP, Type::Number(NumericType::Float));

#[derive(Debug, Clone)]
pub enum Blend {
    SmoothUnion { prop: PropertyIdentifier, k: Expr },
    SmoothIntersection { prop: PropertyIdentifier, k: Expr },
    SmoothSubtraction { prop: PropertyIdentifier, k: Expr },
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
                Blend::SmoothUnion { prop, .. } => FunctionIdentifier(SMOOTH_UNION.0.concat(prop)),
                Blend::SmoothIntersection { prop, .. } => {
                    FunctionIdentifier(SMOOTH_INTERSECTION.0.concat(prop))
                }
                Blend::SmoothSubtraction { prop, .. } => {
                    FunctionIdentifier(SMOOTH_SUBTRACTION.0.concat(prop))
                }
            },
            public: false,
            inputs: vec![
                InputDefinition {
                    id: K,
                    mutable: false,
                },
                InputDefinition {
                    id: COMBINE_CONTEXT_PROP,
                    mutable: true,
                },
            ],
            output: COMBINE_CONTEXT_PROP,
            block: match self {
                Blend::SmoothUnion { prop, .. } => {
                    let mut block = vec![
                        NUM.bind(
                            (0.5.literal()
                                + 0.5.literal()
                                    * ([COMBINE_CONTEXT_PROP, RIGHT, DISTANCE].read()
                                        - [COMBINE_CONTEXT_PROP, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(0.0.literal())
                            .min(1.0.literal()),
                        ),
                        [COMBINE_CONTEXT_PROP, OUT, prop.clone()].write(
                            [COMBINE_CONTEXT_PROP, RIGHT, prop.clone()].read().mix(
                                [COMBINE_CONTEXT_PROP, LEFT, prop.clone()].read(),
                                NUM.read(),
                            ),
                        ),
                    ];

                    if *prop == DISTANCE {
                        block.push([COMBINE_CONTEXT_PROP, OUT, DISTANCE].write(
                            [COMBINE_CONTEXT_PROP, OUT, DISTANCE].read()
                                - K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT_PROP.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { prop, .. } => {
                    let property = prop.clone();

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
                    let property = prop.clone();

                    let mut block = vec![
                        NUM.bind(
                            (0.5.literal()
                                - 0.5.literal()
                                    * ([COMBINE_CONTEXT_PROP, RIGHT, DISTANCE].read()
                                        + [COMBINE_CONTEXT_PROP, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(0.0.literal())
                            .min(1.0.literal()),
                        ),
                        [COMBINE_CONTEXT_PROP, OUT, property.clone()].write(
                            [COMBINE_CONTEXT_PROP, LEFT, property.clone()].read().mix(
                                -[COMBINE_CONTEXT_PROP, RIGHT, property.clone()].read(),
                                NUM.read(),
                            ),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push([COMBINE_CONTEXT_PROP, OUT, DISTANCE].write(
                            [COMBINE_CONTEXT_PROP, OUT, DISTANCE].read()
                                + K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT_PROP.read().output());

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
                function: FunctionIdentifier(SMOOTH_UNION.0.concat(prop)),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothIntersection { prop, k } => elysian_core::ir::ast::Expr::Call {
                function: FunctionIdentifier(SMOOTH_INTERSECTION.0.concat(prop)),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothSubtraction { prop, k } => elysian_core::ir::ast::Expr::Call {
                function: FunctionIdentifier(SMOOTH_SUBTRACTION.0.concat(prop)),
                args: vec![k.clone().into(), input],
            },
        }
    }
}
