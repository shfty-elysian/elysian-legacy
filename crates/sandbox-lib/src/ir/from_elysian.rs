use std::fmt::Debug;

use rust_gpu_bridge::{One, Two, Zero};

use crate::{
    elysian::{
        combinator::{Blend, Boolean, Combinator},
        Elysian, Field, Modifier,
    },
    ir::{
        ast::{
            Block, ComposeBlocks, Expr, IntoBlock, IntoLiteral, IntoRead, IntoValue, IntoWrite,
            Property,
            Stmt::{self, *},
            COLOR, DISTANCE, ERROR, GRADIENT, LEFT, LIGHT, NUM, OUT, POSITION, RIGHT, SUPPORT,
            TANGENT, TIME, UV, VECT,
        },
        module::{
            FieldDefinition, FunctionDefinition, InputDefinition, Module, StructDefinition, Type,
        },
    },
};

use super::ast::{COMBINE_CONTEXT, CONTEXT, K};

pub fn elysian_struct_definitions<N, V>(elysian: &Elysian<N, V>) -> Vec<StructDefinition> {
    vec![
        StructDefinition {
            name: "Context",
            public: true,
            fields: [
                (POSITION, FieldDefinition { public: true }),
                (TIME, FieldDefinition { public: true }),
                (DISTANCE, FieldDefinition { public: true }),
                (GRADIENT, FieldDefinition { public: true }),
                (UV, FieldDefinition { public: true }),
                (TANGENT, FieldDefinition { public: true }),
                (COLOR, FieldDefinition { public: true }),
                (LIGHT, FieldDefinition { public: true }),
                (SUPPORT, FieldDefinition { public: true }),
                (ERROR, FieldDefinition { public: true }),
                (NUM, FieldDefinition { public: true }),
                (VECT, FieldDefinition { public: true }),
            ]
            .into_iter()
            .collect(),
        },
        StructDefinition {
            name: "CombineContext",
            public: false,
            fields: [
                (LEFT, FieldDefinition { public: false }),
                (RIGHT, FieldDefinition { public: false }),
                (OUT, FieldDefinition { public: false }),
            ]
            .into_iter()
            .collect(),
        },
    ]
}

pub fn elysian_module<N, V>(elysian: &Elysian<N, V>) -> Module<N, V>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    let mut functions = elysian_functions(elysian);
    functions.sort_by(|lhs, rhs| lhs.name.cmp(rhs.name));
    functions.dedup_by(|lhs, rhs| lhs.name == rhs.name);

    let struct_definitions = elysian_struct_definitions(elysian);
    let entry_point = FunctionDefinition {
        name: "shape",
        public: true,
        inputs: [(CONTEXT, InputDefinition { mutable: false })]
            .into_iter()
            .collect(),
        output: Type::Struct("Context"),
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
        Elysian::Modifier(m) => vec![modifier_expr(&m).output()],
        Elysian::Combine { combinator, shapes } => {
            let mut stmts = elysian_entry_point_combine(combinator, shapes);
            stmts.push(Stmt::Output([COMBINE_CONTEXT, OUT].read()));
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
                "COMBINE_CONTEXT",
                [(OUT, Box::new(field_expr(f)))].into(),
            ))]);
        }
        Elysian::Modifier(m) => {
            block.extend([COMBINE_CONTEXT.write(Expr::Construct(
                "COMBINE_CONTEXT",
                [(OUT, Box::new(modifier_expr(m)))].into(),
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
                "COMBINE_CONTEXT",
                [
                    (LEFT, Box::new([COMBINE_CONTEXT, OUT].read())),
                    (RIGHT, Box::new(elysian_expr(&next))),
                ]
                .into(),
            )),
            COMBINE_CONTEXT.write(combinator.clone()),
        ]);

        acc
    })
}

pub fn field_expr<N, V>(field: &Field<N, V>) -> Expr<N, V> {
    match field {
        Field::Point => Expr::Call {
            function: "point",
            args: vec![Box::new(CONTEXT.read())],
        },
        Field::_Phantom(_) => unimplemented!(),
    }
}

pub fn modifier_expr<N, V>(modifier: &Modifier<N, V>) -> Expr<N, V>
where
    N: Clone + IntoValue<N, V>,
    V: Clone + IntoValue<N, V>,
{
    match modifier {
        Modifier::Translate { delta, shape } => Expr::Call {
            function: "translate",
            args: vec![
                Box::new(delta.clone().into()),
                Box::new(elysian_expr(shape)),
            ],
        },
        Modifier::Elongate { dir, shape, .. } => Expr::Call {
            function: "elongate",
            args: vec![Box::new(dir.clone().into()), Box::new(elysian_expr(shape))],
        },
        Modifier::Isosurface { dist, shape } => Expr::Call {
            function: "isosurface",
            args: vec![Box::new(dist.clone().into()), Box::new(elysian_expr(shape))],
        },
        Modifier::Manifold { shape } => Expr::Call {
            function: "manifold",
            args: vec![Box::new(elysian_expr(shape))],
        },
    }
}

pub fn boolean_expr<N, V>(boolean: &Boolean) -> Expr<N, V> {
    match boolean {
        Boolean::Union => Expr::Call {
            function: "union",
            args: vec![],
        },
        Boolean::Intersection => Expr::Call {
            function: "intersection",
            args: vec![],
        },
        Boolean::Subtraction => Expr::Call {
            function: "subtraction",
            args: vec![],
        },
    }
}

pub fn blend_expr<N, V>(blend: &Blend<N, V>) -> Expr<N, V>
where
    N: Clone,
    V: Clone,
{
    match blend {
        Blend::SmoothUnion { attr, k } => Expr::Call {
            function: "smooth_union",
            args: vec![Box::new(k.clone().into())],
        },
        Blend::SmoothIntersection { attr, k } => Expr::Call {
            function: "smooth_intersection",
            args: vec![Box::new(k.clone().into())],
        },
        Blend::SmoothSubtraction { attr, k } => Expr::Call {
            function: "smooth_subtraction",
            args: vec![Box::new(k.clone().into())],
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

            args.push(Box::new(acc));

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
        Elysian::Modifier(m) => modifier_expr(m),
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
                name: match b {
                    Boolean::Union => "union",
                    Boolean::Intersection => "intersection",
                    Boolean::Subtraction => "subtraction",
                },
                public: false,
                inputs: [(COMBINE_CONTEXT, InputDefinition { mutable: true })]
                    .into_iter()
                    .collect(),
                output: Type::Struct("CombineContext"),
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
                                [COMBINE_CONTEXT, OUT].write([COMBINE_CONTEXT, RIGHT].read()),
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
                                Nop,
                            ),
                        COMBINE_CONTEXT.read().output(),
                    ]
                    .block(),
                },
            }
        }

        Combinator::Blend(b) => FunctionDefinition {
            name: match b {
                Blend::SmoothUnion { .. } => "smooth_union",
                Blend::SmoothIntersection { .. } => "smooth_intersection",
                Blend::SmoothSubtraction { .. } => "smooth_subtraction",
            },
            public: false,
            inputs: [
                (K, InputDefinition { mutable: false }),
                (COMBINE_CONTEXT, InputDefinition { mutable: true }),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("COMBINE_CONTEXT"),
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
            name: "point",
            public: false,
            inputs: [(CONTEXT, InputDefinition { mutable: true })]
                .into_iter()
                .collect(),
            output: Type::Struct("Context"),
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

pub fn modifier_function<N, V>(modifier: &Modifier<N, V>) -> FunctionDefinition<N, V>
where
    N: Clone,
    V: Clone,
{
    match modifier {
        Modifier::Translate { .. } => FunctionDefinition {
            name: "translate",
            public: false,
            inputs: [
                (VECT, InputDefinition { mutable: false }),
                (CONTEXT, InputDefinition { mutable: true }),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                [CONTEXT, POSITION].write([CONTEXT, POSITION].read() - VECT.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
        Modifier::Elongate { infinite, .. } => FunctionDefinition {
            name: "elongate",
            public: false,
            inputs: [
                (VECT, InputDefinition { mutable: false }),
                (CONTEXT, InputDefinition { mutable: true }),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
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
        Modifier::Isosurface { .. } => FunctionDefinition {
            name: "isosurface",
            public: false,
            inputs: [
                (NUM, InputDefinition { mutable: false }),
                (CONTEXT, InputDefinition { mutable: true }),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                [CONTEXT, DISTANCE].write([CONTEXT, DISTANCE].read() - NUM.read()),
                CONTEXT.read().output(),
            ]
            .block(),
        },
        Modifier::Manifold { .. } => FunctionDefinition {
            name: "manifold",
            public: false,
            inputs: [(CONTEXT, InputDefinition { mutable: true })]
                .into_iter()
                .collect(),
            output: Type::Struct("Context"),
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
        Elysian::Modifier(m) => modifier_functions(m),
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

fn modifier_functions<N, V>(modifier: &Modifier<N, V>) -> Vec<FunctionDefinition<N, V>>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    match modifier {
        Modifier::Translate { shape, .. } => [modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
        Modifier::Elongate { shape, .. } => [modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
        Modifier::Isosurface { shape, .. } => [modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
        Modifier::Manifold { shape } => [modifier_function(modifier)]
            .into_iter()
            .chain(elysian_functions(shape).into_iter())
            .collect(),
    }
}

pub fn elysian_stmt<N, V>(elysian: &Elysian<N, V>) -> Stmt<N, V>
where
    N: Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: Debug + Clone + IntoValue<N, V>,
{
    match elysian {
        Elysian::Field(field) => match field {
            Field::Point => Stmt::Block(
                [
                    DISTANCE.write(POSITION.read().length()),
                    GRADIENT.write(POSITION.read().normalize()),
                ]
                .block(),
            ),
            _ => unimplemented!(),
        },
        Elysian::Modifier(modifier) => match modifier {
            Modifier::Translate { delta, shape } => {
                let Stmt::Block(block) = elysian_stmt(shape) else {
                    panic!("Elysian statement is not a Block");
                };
                [POSITION.write(POSITION.read() - delta.clone().into())].compose(block)
            }
            Modifier::Elongate {
                dir,
                infinite,
                shape,
            } => {
                let Stmt::Block(block) = elysian_stmt(shape) else {
                    panic!("Elysian statement is not a Block");
                };

                let expr = POSITION.read().dot(NUM.read().normalize());

                [
                    VECT.write(dir.clone().into()),
                    POSITION.write(
                        POSITION.read()
                            - VECT.read().normalize()
                                * if *infinite {
                                    expr
                                } else {
                                    expr.max(-NUM.read().length()).min(NUM.read().length())
                                },
                    ),
                ]
                .compose(block)
            }
            Modifier::Isosurface { dist, shape } => {
                let Stmt::Block(b) = elysian_stmt(shape) else {
                    panic!("Elysian statement is not a Block");
                };

                b.compose(
                    [
                        NUM.write(dist.clone().into()),
                        DISTANCE.write(DISTANCE.read() - NUM.read()),
                    ]
                    .block(),
                )
            }
            Modifier::Manifold { shape } => {
                let Stmt::Block(b) = elysian_stmt(shape) else {
                    panic!("Elysian statement is not a Block");
                };

                b.compose(
                    [
                        NUM.write(DISTANCE.read()),
                        DISTANCE.write(NUM.read().abs()),
                        GRADIENT.write(GRADIENT.read() * NUM.read().sign()),
                    ]
                    .block(),
                )
            }
        },
        Elysian::Combine { .. } => unimplemented!(),
        Elysian::Alias(_) => panic!("Aliases must be expanded before conversion to Ast"),
    }
}
