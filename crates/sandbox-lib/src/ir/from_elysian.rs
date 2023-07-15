use std::fmt::Debug;

use rust_gpu_bridge::{One, Two, Zero};

use crate::{
    elysian::{
        combinator::{Blend, Boolean, Combinator},
        Elysian, Field, PostModifier, PreModifier,
    },
    ir::{
        ast::{
            Block, Expr, IntoBlock, IntoLiteral, IntoRead, IntoValue, IntoWrite, Property, Stmt,
            COLOR, COMBINE_CONTEXT, CONTEXT, DISTANCE, ERROR, GRADIENT, K, LEFT, LIGHT, NUM, OUT,
            POSITION, RIGHT, SUPPORT, TANGENT, TIME, UV, VECT,
        },
        module::{FieldDefinition, FunctionDefinition, InputDefinition, Module, StructDefinition},
    },
};

use super::ast::Identifier;

pub const CONTEXT_STRUCT: StructDefinition = StructDefinition {
    id: Identifier::new("Context", 1198218077110787867),
    public: true,
    fields: &[
        FieldDefinition {
            prop: POSITION,
            public: true,
        },
        FieldDefinition {
            prop: TIME,
            public: true,
        },
        FieldDefinition {
            prop: DISTANCE,
            public: true,
        },
        FieldDefinition {
            prop: GRADIENT,
            public: true,
        },
        FieldDefinition {
            prop: UV,
            public: true,
        },
        FieldDefinition {
            prop: TANGENT,
            public: true,
        },
        FieldDefinition {
            prop: COLOR,
            public: true,
        },
        FieldDefinition {
            prop: LIGHT,
            public: true,
        },
        FieldDefinition {
            prop: SUPPORT,
            public: true,
        },
        FieldDefinition {
            prop: ERROR,
            public: true,
        },
        FieldDefinition {
            prop: NUM,
            public: true,
        },
        FieldDefinition {
            prop: VECT,
            public: true,
        },
    ],
};

pub const COMBINE_CONTEXT_STRUCT: StructDefinition = StructDefinition {
    id: Identifier::new("CombineContext", 416045102551943616),
    public: false,
    fields: &[
        FieldDefinition {
            prop: LEFT,
            public: false,
        },
        FieldDefinition {
            prop: RIGHT,
            public: false,
        },
        FieldDefinition {
            prop: OUT,
            public: false,
        },
    ],
};

pub fn elysian_struct_definitions<N, V>(elysian: &Elysian<N, V>) -> Vec<StructDefinition> {
    vec![CONTEXT_STRUCT, COMBINE_CONTEXT_STRUCT]
}

pub fn elysian_module<N, V>(elysian: &Elysian<N, V>) -> Module<N, V>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    let mut functions = elysian_functions(elysian);
    functions.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));
    functions.dedup_by(|lhs, rhs| lhs.id == rhs.id);

    let struct_definitions = elysian_struct_definitions(elysian);
    let entry_point = FunctionDefinition {
        id: Identifier::new("shape", 523056258704924944),
        public: true,
        inputs: &[InputDefinition {
            prop: CONTEXT,
            mutable: false,
        }],
        output: &CONTEXT_STRUCT,
        block: elysian_entry_point(elysian),
    };
    Module {
        entry_point,
        struct_definitions,
        function_definitions: functions,
    }
}

pub fn elysian_entry_point<N, V>(elysian: &Elysian<N, V>) -> Block<N, V>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    Block(match elysian {
        Elysian::Field(f) => vec![field_expr(&f).output()],
        Elysian::PreModifier(m) => vec![pre_modifier_expr(&m).output()],
        Elysian::PostModifier(m) => vec![post_modifier_expr(&m).output()],
        Elysian::Combine { combinator, shapes } => {
            let mut stmts = elysian_entry_point_combine(combinator, shapes);
            stmts.push([COMBINE_CONTEXT, OUT].read().output());
            stmts
        }
        Elysian::Alias(_) => {
            unimplemented!("Aliases must be expanded before conversion to Block")
        }
    })
}

pub fn elysian_entry_point_combine<N, V>(
    combinator: &Vec<Combinator<N, V>>,
    shapes: &Vec<Elysian<N, V>>,
) -> Vec<Stmt<N, V>>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    let combinator = combinator_list_expr(combinator);

    let mut block = vec![];

    let mut iter = shapes.iter();

    let lhs = iter.next().expect("No LHS shape");

    match lhs {
        Elysian::Field(f) => {
            block.extend([COMBINE_CONTEXT.write(Expr::Construct(
                &COMBINE_CONTEXT_STRUCT,
                [(OUT, field_expr(f))].into(),
            ))]);
        }
        Elysian::PreModifier(m) => {
            block.extend([COMBINE_CONTEXT.write(Expr::Construct(
                &COMBINE_CONTEXT_STRUCT,
                [(OUT, pre_modifier_expr(m))].into(),
            ))]);
        }
        Elysian::PostModifier(m) => {
            block.extend([COMBINE_CONTEXT.write(Expr::Construct(
                &COMBINE_CONTEXT_STRUCT,
                [(OUT, post_modifier_expr(m))].into(),
            ))]);
        }
        Elysian::Combine { combinator, shapes } => {
            block.extend(elysian_entry_point_combine(combinator, shapes));
        }
        Elysian::Alias(_) => {
            panic!("Aliases must be expanded before conversion to entry point")
        }
    }

    iter.fold(block, |mut acc, next| {
        acc.extend([
            COMBINE_CONTEXT.write(Expr::Construct(
                &COMBINE_CONTEXT_STRUCT,
                [
                    (LEFT, [COMBINE_CONTEXT, OUT].read()),
                    (RIGHT, elysian_expr(&next)),
                ]
                .into(),
            )),
            COMBINE_CONTEXT.write(combinator.clone()),
        ]);

        acc
    })
}

const POINT: Identifier = Identifier::new("point", 419357041369711478);

pub fn field_expr<N, V>(field: &Field<N, V>) -> Expr<N, V> {
    match field {
        Field::Point => Expr::Call {
            function: POINT,
            args: vec![CONTEXT.read()],
        },
        Field::_Phantom(_) => unimplemented!(),
    }
}

const TRANSLATE: Identifier = Identifier::new("translate", 419357041369711478);
const ELONGATE: Identifier = Identifier::new("elongate", 1022510703206415324);
const ISOSURFACE: Identifier = Identifier::new("isosurface", 1163045471729794054);
const MANIFOLD: Identifier = Identifier::new("manifold", 7861274791729269697);

pub fn pre_modifier_expr<N, V>(modifier: &PreModifier<N, V>) -> Expr<N, V>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    match modifier {
        PreModifier::Translate { delta, shape } => {
            let Expr::Call { function, mut args } = elysian_expr(shape) else {
                panic!("Invalid Translate inner");
            };

            let arg = args.pop().expect("Empty arg list");

            args.push(Expr::Call {
                function: TRANSLATE,
                args: vec![delta.clone().into(), arg],
            });

            Expr::Call { function, args }
        }
        PreModifier::Elongate { dir, shape, .. } => {
            let Expr::Call { function, mut args } = elysian_expr(shape) else {
                panic!("Invalid Translate inner");
            };

            let arg = args.pop().expect("Empty arg list");

            args.push(Expr::Call {
                function: ELONGATE,
                args: vec![dir.clone().into(), arg],
            });

            Expr::Call { function, args }
        }
    }
}

pub fn post_modifier_expr<N, V>(modifier: &PostModifier<N, V>) -> Expr<N, V>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    match modifier {
        PostModifier::Isosurface { dist, shape } => Expr::Call {
            function: ISOSURFACE,
            args: vec![dist.clone().into(), elysian_expr(shape)],
        },
        PostModifier::Manifold { shape } => Expr::Call {
            function: MANIFOLD,
            args: vec![elysian_expr(shape)],
        },
    }
}

const UNION: Identifier = Identifier::new("union", 1894363406191409858);
const INTERSECTION: Identifier = Identifier::new("intersection", 18033822391797795038);
const SUBTRACTION: Identifier = Identifier::new("subtraction", 1414822549598552032);

pub fn boolean_expr<N, V>(boolean: &Boolean) -> Expr<N, V> {
    match boolean {
        Boolean::Union => Expr::Call {
            function: UNION,
            args: vec![],
        },
        Boolean::Intersection => Expr::Call {
            function: INTERSECTION,
            args: vec![],
        },
        Boolean::Subtraction => Expr::Call {
            function: SUBTRACTION,
            args: vec![],
        },
    }
}

const SMOOTH_UNION: Identifier = Identifier::new("smooth_union", 1894363406191409858);
const SMOOTH_INTERSECTION: Identifier =
    Identifier::new("smooth_intersection", 18033822391797795038);
const SMOOTH_SUBTRACTION: Identifier = Identifier::new("smooth_subtraction", 1414822549598552032);

pub fn blend_expr<N, V>(blend: &Blend<N, V>) -> Expr<N, V>
where
    N: Clone,
    V: Clone,
{
    match blend {
        Blend::SmoothUnion { attr, k } => Expr::Call {
            function: SMOOTH_UNION,
            args: vec![k.clone().into()],
        },
        Blend::SmoothIntersection { attr, k } => Expr::Call {
            function: SMOOTH_INTERSECTION,
            args: vec![k.clone().into()],
        },
        Blend::SmoothSubtraction { attr, k } => Expr::Call {
            function: SMOOTH_SUBTRACTION,
            args: vec![k.clone().into()],
        },
    }
}

pub fn combinator_expr<N, V>(combinator: &Combinator<N, V>) -> Expr<N, V>
where
    N: Clone,
    V: Clone,
{
    match combinator {
        Combinator::Boolean(b) => boolean_expr(b),
        Combinator::Blend(b) => blend_expr(b),
    }
}

pub fn combinator_list_expr<'a, I: IntoIterator<Item = &'a Combinator<N, V>>, N: 'a, V: 'a>(
    combinators: I,
) -> Expr<N, V>
where
    N: Clone,
    V: Clone,
{
    combinators
        .into_iter()
        .fold(COMBINE_CONTEXT.read(), |acc: Expr<N, V>, next| {
            let Expr::Call{ function, mut args } = combinator_expr(next) else  {
                            panic!("Combinator expression is not a CallResult")
                        };

            args.push(acc);

            Expr::Call { function, args }
        })
}

pub fn elysian_expr<N, V>(elysian: &Elysian<N, V>) -> Expr<N, V>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    match elysian {
        Elysian::Field(f) => field_expr(f),
        Elysian::PreModifier(m) => pre_modifier_expr(m),
        Elysian::PostModifier(m) => post_modifier_expr(m),
        Elysian::Combine { combinator, .. } => combinator_list_expr(combinator),
        Elysian::Alias(_) => panic!("Aliases must be expanded before conversion to Expr"),
    }
}

pub fn combinator_function<N, V>(combinator: &Combinator<N, V>) -> FunctionDefinition<N, V>
where
    N: Clone + One + Two + Zero + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    match combinator {
        Combinator::Boolean(b) => {
            FunctionDefinition {
                id: match b {
                    Boolean::Union => UNION,
                    Boolean::Intersection => INTERSECTION,
                    Boolean::Subtraction => SUBTRACTION,
                },
                public: false,
                inputs: &[InputDefinition {
                    prop: COMBINE_CONTEXT,
                    mutable: true,
                }],
                output: &COMBINE_CONTEXT_STRUCT,
                block: match b {
                    Boolean::Union | Boolean::Intersection => [
                        [COMBINE_CONTEXT, OUT]
                            .write([COMBINE_CONTEXT, LEFT].read())
                            .if_else(
                                match b {
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
                    .block(),
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
            }
        }

        Combinator::Blend(b) => FunctionDefinition {
            id: match b {
                Blend::SmoothUnion { .. } => SMOOTH_UNION,
                Blend::SmoothIntersection { .. } => SMOOTH_INTERSECTION,
                Blend::SmoothSubtraction { .. } => SMOOTH_SUBTRACTION,
            },
            public: false,
            inputs: &[
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
            block: match b {
                Blend::SmoothUnion { attr, .. } => {
                    let property: Property = (*attr).into();

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
                        [COMBINE_CONTEXT, OUT, property].write(
                            [COMBINE_CONTEXT, RIGHT, property]
                                .read()
                                .mix([COMBINE_CONTEXT, LEFT, property].read(), NUM.read()),
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
                    let property: Property = (*attr).into();

                    let mut block = vec![
                        NUM.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                - (N::ONE.literal() / N::TWO.literal())
                                    * ([RIGHT, DISTANCE].read() - [LEFT, DISTANCE].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
                        ),
                        property.write(
                            [RIGHT, property]
                                .read()
                                .mix([LEFT, property].read(), NUM.read()),
                        ),
                    ];

                    if property == DISTANCE {
                        block.push(DISTANCE.write(
                            DISTANCE.read()
                                + K.read() * NUM.read() * (N::ONE.literal() - NUM.read()),
                        ))
                    }

                    block.push([OUT, property].write([OUT, property].read()));

                    block.into_iter().collect()
                }
                Blend::SmoothSubtraction { attr, .. } => {
                    let property: Property = (*attr).into();

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
                        [COMBINE_CONTEXT, OUT, property].write(
                            [COMBINE_CONTEXT, LEFT, property]
                                .read()
                                .mix(-[COMBINE_CONTEXT, RIGHT, property].read(), NUM.read()),
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
        },
    }
}

pub fn field_function<N, V>(field: &Field<N, V>) -> FunctionDefinition<N, V> {
    match field {
        Field::Point => FunctionDefinition {
            id: POINT,
            public: false,
            inputs: &[InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, POSITION].read().length()),
                [CONTEXT, GRADIENT].write([CONTEXT, POSITION].read().normalize()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
        Field::_Phantom(_) => unimplemented!(),
    }
}

pub fn pre_modifier_function<N, V>(modifier: &PreModifier<N, V>) -> FunctionDefinition<N, V>
where
    N: Clone,
    V: Clone,
{
    match modifier {
        PreModifier::Translate { .. } => FunctionDefinition {
            id: TRANSLATE,
            public: false,
            inputs: &[
                InputDefinition {
                    prop: VECT,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, POSITION].write([CONTEXT, POSITION].read() - VECT.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
        PreModifier::Elongate { infinite, .. } => FunctionDefinition {
            id: ELONGATE,
            public: false,
            inputs: &[
                InputDefinition {
                    prop: VECT,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: {
                let expr = [CONTEXT, POSITION].read().dot(VECT.read().normalize());

                [
                    [CONTEXT, POSITION].write(
                        [CONTEXT, POSITION].read()
                            - VECT.read().normalize()
                                * if *infinite {
                                    expr
                                } else {
                                    expr.max(-VECT.read().length()).min(VECT.read().length())
                                },
                    ),
                    CONTEXT.read().output(),
                ]
                .block()
            },
        },
    }
}

pub fn post_modifier_function<N, V>(modifier: &PostModifier<N, V>) -> FunctionDefinition<N, V>
where
    N: Clone,
    V: Clone,
{
    match modifier {
        PostModifier::Isosurface { .. } => FunctionDefinition {
            id: ISOSURFACE,
            public: false,
            inputs: &[
                InputDefinition {
                    prop: NUM,
                    mutable: false,
                },
                InputDefinition {
                    prop: CONTEXT,
                    mutable: true,
                },
            ],
            output: &CONTEXT_STRUCT,
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - NUM.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
        PostModifier::Manifold { .. } => FunctionDefinition {
            id: MANIFOLD,
            public: false,
            inputs: &[InputDefinition {
                prop: CONTEXT,
                mutable: true,
            }],
            output: &CONTEXT_STRUCT,
            block: [
                NUM.write([CONTEXT, DISTANCE].read()),
                [CONTEXT, DISTANCE].write(NUM.read().abs()),
                [CONTEXT, GRADIENT].write([CONTEXT, GRADIENT].read() * NUM.read().sign()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
    }
}

pub fn elysian_functions<N, V>(elysian: &Elysian<N, V>) -> Vec<FunctionDefinition<N, V>>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    match elysian {
        Elysian::Field(f) => vec![field_function(f)],
        Elysian::PreModifier(m) => pre_modifier_functions(m),
        Elysian::PostModifier(m) => post_modifier_functions(m),
        Elysian::Combine { combinator, shapes } => combinator
            .iter()
            .map(combinator_function)
            .chain(shapes.iter().map(elysian_functions).flatten())
            .collect(),
        Elysian::Alias(_) => {
            unimplemented!("Aliases must be expanded before conversion to Functions")
        }
    }
}

fn pre_modifier_functions<N, V>(modifier: &PreModifier<N, V>) -> Vec<FunctionDefinition<N, V>>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    match modifier {
        PreModifier::Translate { shape, .. } => [pre_modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
        PreModifier::Elongate { shape, .. } => [pre_modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
    }
}

fn post_modifier_functions<N, V>(modifier: &PostModifier<N, V>) -> Vec<FunctionDefinition<N, V>>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    match modifier {
        PostModifier::Isosurface { shape, .. } => [post_modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
        PostModifier::Manifold { shape } => [post_modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
    }
}
