use std::fmt::Debug;

use elysian_core::{
    ast::{attribute::Attribute, combine::COMBINE_CONTEXT_STRUCT, expr::Expr},
    ir::{
        as_ir::{AsIR, Domains},
        ast::{
            Identifier, IntoLiteral, IntoRead, IntoWrite, Property, COMBINE_CONTEXT, DISTANCE,
            LEFT, NUM, OUT, RIGHT,
        },
        module::{FunctionDefinition, InputDefinition, SpecializationData, Type},
    },
};

pub const SMOOTH_UNION: Identifier = Identifier::new("smooth_union", 1894363406191409858);
pub const SMOOTH_INTERSECTION: Identifier =
    Identifier::new("smooth_intersection", 18033822391797795038);
pub const SMOOTH_SUBTRACTION: Identifier =
    Identifier::new("smooth_subtraction", 1414822549598552032);

pub const K: Property = Property::new("k", Type::Number, 12632115441234896764);

pub enum Blend {
    SmoothUnion { attr: Attribute, k: Expr },
    SmoothIntersection { attr: Attribute, k: Expr },
    SmoothSubtraction { attr: Attribute, k: Expr },
}

impl Debug for Blend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SmoothUnion { attr, k } => f
                .debug_struct("SmoothUnion")
                .field("attr", attr)
                .field("k", k)
                .finish(),
            Self::SmoothIntersection { attr, k } => f
                .debug_struct("SmoothIntersection")
                .field("attr", attr)
                .field("k", k)
                .finish(),
            Self::SmoothSubtraction { attr, k } => f
                .debug_struct("SmoothSubtraction")
                .field("attr", attr)
                .field("k", k)
                .finish(),
        }
    }
}

impl Clone for Blend {
    fn clone(&self) -> Self {
        match self {
            Self::SmoothUnion { attr, k } => Self::SmoothUnion {
                attr: attr.clone(),
                k: k.clone(),
            },
            Self::SmoothIntersection { attr, k } => Self::SmoothIntersection {
                attr: attr.clone(),
                k: k.clone(),
            },
            Self::SmoothSubtraction { attr, k } => Self::SmoothSubtraction {
                attr: attr.clone(),
                k: k.clone(),
            },
        }
    }
}

impl std::hash::Hash for Blend {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Blend::SmoothUnion { attr, k } => {
                attr.hash(state);
                k.hash(state);
            }
            Blend::SmoothIntersection { attr, k } => {
                attr.hash(state);
                k.hash(state);
            }
            Blend::SmoothSubtraction { attr, k } => {
                attr.hash(state);
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
                Blend::SmoothUnion { attr, .. } => SMOOTH_UNION.concat(Property::from(*attr).id()),
                Blend::SmoothIntersection { attr, .. } => {
                    SMOOTH_INTERSECTION.concat(Property::from(*attr).id())
                }
                Blend::SmoothSubtraction { attr, .. } => {
                    SMOOTH_SUBTRACTION.concat(Property::from(*attr).id())
                }
            },
            public: false,
            inputs: vec![
                InputDefinition {
                    prop: K,
                    mutable: false,
                },
                InputDefinition {
                    prop: COMBINE_CONTEXT,
                    mutable: true,
                },
            ],
            output: &COMBINE_CONTEXT_STRUCT,
            block: match self {
                Blend::SmoothUnion { attr, .. } => {
                    let property: Property = attr.clone().into();

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
                        [COMBINE_CONTEXT, OUT, property.clone()].write(
                            [COMBINE_CONTEXT, RIGHT, property.clone()]
                                .read()
                                .mix([COMBINE_CONTEXT, LEFT, property.clone()].read(), NUM.read()),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push([COMBINE_CONTEXT, OUT, DISTANCE].write(
                            [COMBINE_CONTEXT, OUT, DISTANCE].read()
                                - K.read() * NUM.read() * (1.0.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { attr, .. } => {
                    let property: Property = attr.clone().into();

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
                Blend::SmoothSubtraction { attr, .. } => {
                    let property: Property = attr.clone().into();

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
            Blend::SmoothUnion { attr, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_UNION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothIntersection { attr, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_INTERSECTION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothSubtraction { attr, k } => elysian_core::ir::ast::Expr::Call {
                function: SMOOTH_SUBTRACTION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
        }
    }
}
