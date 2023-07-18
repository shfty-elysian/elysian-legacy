use std::fmt::Debug;

use rust_gpu_bridge::{One, Two, Zero};

use crate::{
    ast::{attribute::Attribute, expr::Expr, combine::COMBINE_CONTEXT_STRUCT},
    ir::{
        as_ir::AsIR,
        ast::{
            Identifier, IntoLiteral, IntoRead, IntoValue, IntoWrite, Property, TypeSpec,
            COMBINE_CONTEXT, DISTANCE, LEFT, NUM, OUT, RIGHT, VectorSpace,
        },
        module::{FunctionDefinition, InputDefinition, Type},
    },
};

pub const SMOOTH_UNION: Identifier = Identifier::new("smooth_union", 1894363406191409858);
pub const SMOOTH_INTERSECTION: Identifier =
    Identifier::new("smooth_intersection", 18033822391797795038);
pub const SMOOTH_SUBTRACTION: Identifier =
    Identifier::new("smooth_subtraction", 1414822549598552032);

pub const K: Property = Property::new("k", Type::Number, 12632115441234896764);

pub enum Blend<T>
where
    T: TypeSpec,
{
    SmoothUnion { attr: Attribute, k: Expr<T> },
    SmoothIntersection { attr: Attribute, k: Expr<T> },
    SmoothSubtraction { attr: Attribute, k: Expr<T> },
}

impl<T> Debug for Blend<T>
where
    T: TypeSpec,
{
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

impl<T> Clone for Blend<T>
where
    T: TypeSpec,
{
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

impl<T> std::hash::Hash for Blend<T>
where
    T: TypeSpec,
{
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

impl<T, const N: usize> AsIR<T, N> for Blend<T>
where
    T: TypeSpec + VectorSpace<N>,
    T::NUMBER: IntoValue<T, N>,
    T::VECTOR2: IntoValue<T, N>,
{
    fn functions(&self) -> Vec<crate::ir::module::FunctionDefinition<T, N>> {
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
                            ((T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                + (T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        - [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(T::NUMBER::ZERO.literal())
                            .min(T::NUMBER::ONE.literal()),
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
                                - K.read() * NUM.read() * (T::NUMBER::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { attr, .. } => {
                    let property: Property = attr.clone().into();

                    let mut block = vec![
                        NUM.write(
                            ((T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                - (T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                    * ([RIGHT, DISTANCE].read() - [LEFT, DISTANCE].read())
                                    / K.read())
                            .max(T::NUMBER::ZERO.literal())
                            .min(T::NUMBER::ONE.literal()),
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
                                + K.read() * NUM.read() * (T::NUMBER::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push([OUT, property.clone()].write([OUT, property.clone()].read()));

                    block.into_iter().collect()
                }
                Blend::SmoothSubtraction { attr, .. } => {
                    let property: Property = attr.clone().into();

                    let mut block = vec![
                        NUM.write(
                            ((T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                - (T::NUMBER::ONE.literal() / T::NUMBER::TWO.literal())
                                    * ([COMBINE_CONTEXT, RIGHT, DISTANCE].read()
                                        + [COMBINE_CONTEXT, LEFT, DISTANCE].read())
                                    / K.read())
                            .max(T::NUMBER::ZERO.literal())
                            .min(T::NUMBER::ONE.literal()),
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
                                + K.read() * NUM.read() * (T::NUMBER::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push(COMBINE_CONTEXT.read().output());

                    block.into_iter().collect()
                }
            },
        }]
    }

    fn expression(&self, input: crate::ir::ast::Expr<T, N>) -> crate::ir::ast::Expr<T, N> {
        match self {
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
        }
    }
}
