use std::fmt::Debug;

use rust_gpu_bridge::{One, Two, Zero};

use crate::{
    ast::{attribute::Attribute, expr::Expr},
    ir::{
        as_ir::{clone_ir, hash_ir, AsIR},
        ast::{
            Identifier, IntoLiteral, IntoRead, IntoValue, IntoWrite, Property, COMBINE_CONTEXT,
            DISTANCE, K, LEFT, NUM, OUT, RIGHT,
        },
        from_elysian::COMBINE_CONTEXT_STRUCT,
        module::{FunctionDefinition, InputDefinition},
    },
};

pub const SMOOTH_UNION: Identifier = Identifier::new("smooth_union", 1894363406191409858);
pub const SMOOTH_INTERSECTION: Identifier =
    Identifier::new("smooth_intersection", 18033822391797795038);
pub const SMOOTH_SUBTRACTION: Identifier =
    Identifier::new("smooth_subtraction", 1414822549598552032);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blend<N, V> {
    SmoothUnion { attr: Attribute, k: Expr<N, V> },
    SmoothIntersection { attr: Attribute, k: Expr<N, V> },
    SmoothSubtraction { attr: Attribute, k: Expr<N, V> },
}

impl<N, V> std::hash::Hash for Blend<N, V> {
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

impl<N, V> AsIR<N, V> for Blend<N, V>
where
    N: 'static + Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: 'static + Debug + Clone + IntoValue<N, V>,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<N, V>> {
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
                        NUM.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                + (N::ONE.literal() / N::TWO.literal())
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        - [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
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
                                - K.read() * NUM.read() * (N::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { attr, .. } => {
                    let property: Property = attr.clone().into();

                    let mut block = vec![
                        NUM.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                - (N::ONE.literal() / N::TWO.literal())
                                    * ([RIGHT, DISTANCE].read() - [LEFT, DISTANCE].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
                        ),
                        property.clone().write(
                            [RIGHT, property.clone()]
                                .read()
                                .mix([LEFT, property.clone()].read(), NUM.read()),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push(DISTANCE.write(
                            DISTANCE.read()
                                + K.read() * NUM.read() * (N::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push([OUT, property.clone()].write([OUT, property.clone()].read()));

                    block.into_iter().collect()
                }
                Blend::SmoothSubtraction { attr, .. } => {
                    let property: Property = attr.clone().into();

                    let mut block = vec![
                        NUM.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                - (N::ONE.literal() / N::TWO.literal())
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        + [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
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
                                + K.read() * NUM.read() * (N::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
            },
        }]
    }

    fn expressions(&self, input: crate::ir::ast::Expr<N, V>) -> Vec<crate::ir::ast::Expr<N, V>> {
        vec![match self {
            Blend::SmoothUnion { attr, k } => crate::ir::ast::Expr::Call {
                function: SMOOTH_UNION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothIntersection { attr, k } => crate::ir::ast::Expr::Call {
                function: SMOOTH_INTERSECTION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
            Blend::SmoothSubtraction { attr, k } => crate::ir::ast::Expr::Call {
                function: SMOOTH_SUBTRACTION.concat(Property::from(*attr).id()),
                args: vec![k.clone().into(), input],
            },
        }]
    }

    fn hash_ir(&self) -> u64 {
        hash_ir(self)
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        clone_ir(self)
    }
}
