use std::fmt::Debug;

use rust_gpu_bridge::{One, Two, Zero};

use crate::{
    elysian::{
        combinator::{Blend, Boolean, Combinator},
        Elysian, Field, Modifier,
    },
    ir::{
        ast::{
            Block, ComposeBlocks, Expr, IntoBlock, IntoLiteral, IntoPathRead, IntoPathWrite,
            IntoValue,
            Property::{self, *},
            Stmt::{self, *},
        },
        module::{
            FieldDefinition, FunctionDefinition, InputDefinition, Module, StructDefinition, Type,
        },
    },
};

pub fn elysian_struct_definitions<N, V>(elysian: &Elysian<N, V>) -> Vec<StructDefinition<N, V>> {
    vec![
        StructDefinition {
            name: "Context",
            public: true,
            fields: [
                (
                    Property::Position,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Time,
                    FieldDefinition {
                        ty: Type::Number,
                        public: true,
                    },
                ),
                (
                    Property::Distance,
                    FieldDefinition {
                        ty: Type::Number,
                        public: true,
                    },
                ),
                (
                    Property::Gradient,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Uv,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Tangent,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Color,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Light,
                    FieldDefinition {
                        ty: Type::Number,
                        public: true,
                    },
                ),
                (
                    Property::Support,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
                (
                    Property::Error,
                    FieldDefinition {
                        ty: Type::Number,
                        public: true,
                    },
                ),
                (
                    Property::Num,
                    FieldDefinition {
                        ty: Type::Number,
                        public: true,
                    },
                ),
                (
                    Property::Vect,
                    FieldDefinition {
                        ty: Type::Vector,
                        public: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        },
        StructDefinition {
            name: "CombineContext",
            public: false,
            fields: [
                (
                    Property::Left,
                    FieldDefinition {
                        ty: Type::Struct("Context"),
                        public: false,
                    },
                ),
                (
                    Property::Right,
                    FieldDefinition {
                        ty: Type::Struct("Context"),
                        public: false,
                    },
                ),
                (
                    Property::Out,
                    FieldDefinition {
                        ty: Type::Struct("Context"),
                        public: false,
                    },
                ),
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
        inputs: [(
            Property::Context,
            InputDefinition {
                ty: Type::Struct("Context"),
                mutable: false,
            },
        )]
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
            stmts.push(Stmt::Output(
                [Property::CombineContext, Property::Out].read(),
            ));
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
            block.extend([Property::CombineContext.write(Expr::Construct(
                "CombineContext",
                [(Property::Out, Box::new(field_expr(f)))].into(),
            ))]);
        }
        Elysian::Modifier(m) => {
            block.extend([Property::CombineContext.write(Expr::Construct(
                "CombineContext",
                [(Property::Out, Box::new(modifier_expr(m)))].into(),
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
            Property::CombineContext.write(Expr::Construct(
                "CombineContext",
                [
                    (
                        Property::Left,
                        Box::new([Property::CombineContext, Property::Out].read()),
                    ),
                    (Property::Right, Box::new(elysian_expr(&next))),
                ]
                .into(),
            )),
            Property::CombineContext.write(combinator.clone()),
        ]);

        acc
    })
}

pub fn field_expr<N, V>(field: &Field<N, V>) -> Expr<N, V> {
    match field {
        Field::Point => Expr::Call {
            function: "point",
            args: vec![Box::new(Property::Context.read())],
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
        .fold(Property::CombineContext.read(), |acc: Expr<N, V>, next| {
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
        Combinator::Boolean(b) => FunctionDefinition {
            name: match b {
                Boolean::Union => "union",
                Boolean::Intersection => "intersection",
                Boolean::Subtraction => "subtraction",
            },
            public: false,
            inputs: [(
                CombineContext,
                InputDefinition {
                    ty: Type::Struct("CombineContext"),
                    mutable: true,
                },
            )]
            .into_iter()
            .collect(),
            output: Type::Struct("CombineContext"),
            block: match b {
                Boolean::Union | Boolean::Intersection => [
                    [CombineContext, Out]
                        .write([CombineContext, Left].read())
                        .if_else(
                            match b {
                                Boolean::Union => [CombineContext, Left, Distance].read().lt([
                                    CombineContext,
                                    Right,
                                    Distance,
                                ]
                                .read()),
                                Boolean::Intersection => [CombineContext, Left, Distance]
                                    .read()
                                    .gt([CombineContext, Right, Distance].read()),
                                _ => unreachable!(),
                            },
                            [CombineContext, Out].write([CombineContext, Right].read()),
                        ),
                    CombineContext.read().output(),
                ]
                .block(),
                Boolean::Subtraction => [
                    [CombineContext, Out].write([CombineContext, Right].read()),
                    [CombineContext, Out, Distance].write(-[CombineContext, Out, Distance].read()),
                    [CombineContext, Out]
                        .write([CombineContext, Left].read())
                        .if_else(
                            [CombineContext, Left, Distance].read().gt([
                                CombineContext,
                                Out,
                                Distance,
                            ]
                            .read()),
                            Nop,
                        ),
                    CombineContext.read().output(),
                ]
                .block(),
            },
        },

        Combinator::Blend(b) => FunctionDefinition {
            name: match b {
                Blend::SmoothUnion { .. } => "smooth_union",
                Blend::SmoothIntersection { .. } => "smooth_intersection",
                Blend::SmoothSubtraction { .. } => "smooth_subtraction",
            },
            public: false,
            inputs: [
                (
                    K,
                    InputDefinition {
                        ty: Type::Number,
                        mutable: false,
                    },
                ),
                (
                    CombineContext,
                    InputDefinition {
                        ty: Type::Struct("CombineContext"),
                        mutable: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("CombineContext"),
            block: match b {
                Blend::SmoothUnion { attr, .. } => {
                    let property: Property = (*attr).into();

                    let mut block = vec![
                        Num.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                + (N::ONE.literal() / N::TWO.literal())
                                    * ([CombineContext, Right, Distance].read()
                                        - [CombineContext, Left, Distance].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
                        ),
                        [CombineContext, Out, property].write(
                            [CombineContext, Right, property]
                                .read()
                                .mix([CombineContext, Left, property].read(), Num.read()),
                        ),
                    ];

                    if property == Distance {
                        block.push([CombineContext, Out, Distance].write(
                            [CombineContext, Out, Distance].read()
                                - K.read() * Num.read() * (N::ONE.literal() - Num.read()),
                        ))
                    }

                    block.push(CombineContext.read().output());

                    block.into_iter().collect()
                }
                Blend::SmoothIntersection { attr, .. } => {
                    let property: Property = (*attr).into();

                    let mut block = vec![
                        Num.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                - (N::ONE.literal() / N::TWO.literal())
                                    * ([Right, Distance].read() - [Left, Distance].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
                        ),
                        property.write(
                            [Right, property]
                                .read()
                                .mix([Left, property].read(), Num.read()),
                        ),
                    ];

                    if property == Distance {
                        block.push(Distance.write(
                            Distance.read()
                                + K.read() * Num.read() * (N::ONE.literal() - Num.read()),
                        ))
                    }

                    block.push([Out, property].write([Out, property].read()));

                    block.into_iter().collect()
                }
                Blend::SmoothSubtraction { attr, .. } => {
                    let property: Property = (*attr).into();

                    let mut block = vec![
                        Num.write(
                            ((N::ONE.literal() / N::TWO.literal())
                                - (N::ONE.literal() / N::TWO.literal())
                                    * ([CombineContext, Right, Distance].read()
                                        + [CombineContext, Left, Distance].read())
                                    / K.read())
                            .max(N::ZERO.literal())
                            .min(N::ONE.literal()),
                        ),
                        [CombineContext, Out, property].write(
                            [CombineContext, Left, property]
                                .read()
                                .mix(-[CombineContext, Right, property].read(), Num.read()),
                        ),
                    ];

                    if property == Distance {
                        block.push([CombineContext, Out, Distance].write(
                            [CombineContext, Out, Distance].read()
                                + K.read() * Num.read() * (N::ONE.literal() - Num.read()),
                        ))
                    }

                    block.push(CombineContext.read().output());

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
            inputs: [(
                Context,
                InputDefinition {
                    ty: Type::Struct("Context"),
                    mutable: true,
                },
            )]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                [Context, Distance].write([Context, Position].read().length()),
                [Context, Gradient].write([Context, Position].read().normalize()),
                Context.read().output(),
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
                (
                    Vect,
                    InputDefinition {
                        ty: Type::Vector,
                        mutable: false,
                    },
                ),
                (
                    Context,
                    InputDefinition {
                        ty: Type::Struct("Context"),
                        mutable: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                [Context, Position].write([Context, Position].read() - Vect.read()),
                Context.read().output(),
            ]
            .block(),
        },
        Modifier::Elongate { infinite, .. } => FunctionDefinition {
            name: "elongate",
            public: false,
            inputs: [
                (
                    Vect,
                    InputDefinition {
                        ty: Type::Vector,
                        mutable: false,
                    },
                ),
                (
                    Context,
                    InputDefinition {
                        ty: Type::Struct("Context"),
                        mutable: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: {
                let expr = [Context, Position].read().dot(Vect.read().normalize());

                [
                    [Context, Position].write(
                        [Context, Position].read()
                            - Vect.read().normalize()
                                * if *infinite {
                                    expr
                                } else {
                                    expr.max(-Vect.read().length()).min(Vect.read().length())
                                },
                    ),
                    Context.read().output(),
                ]
                .block()
            },
        },
        Modifier::Isosurface { .. } => FunctionDefinition {
            name: "isosurface",
            public: false,
            inputs: [
                (
                    Num,
                    InputDefinition {
                        ty: Type::Number,
                        mutable: false,
                    },
                ),
                (
                    Context,
                    InputDefinition {
                        ty: Type::Struct("Context"),
                        mutable: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                [Context, Distance].write([Context, Distance].read() - Num.read()),
                Context.read().output(),
            ]
            .block(),
        },
        Modifier::Manifold { .. } => FunctionDefinition {
            name: "manifold",
            public: false,
            inputs: [(
                Context,
                InputDefinition {
                    ty: Type::Struct("Context"),
                    mutable: true,
                },
            )]
            .into_iter()
            .collect(),
            output: Type::Struct("Context"),
            block: [
                Num.write([Context, Distance].read()),
                [Context, Distance].write(Num.read().abs()),
                [Context, Gradient].write([Context, Gradient].read() * Num.read().sign()),
                Context.read().output(),
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
                    Distance.write(Position.read().length()),
                    Gradient.write(Position.read().normalize()),
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
                [Position.write(Position.read() - delta.clone().into())].compose(block)
            }
            Modifier::Elongate {
                dir,
                infinite,
                shape,
            } => {
                let Stmt::Block(block) = elysian_stmt(shape) else {
                    panic!("Elysian statement is not a Block");
                };

                let expr = Position.read().dot(Num.read().normalize());

                [
                    Vect.write(dir.clone().into()),
                    Position.write(
                        Position.read()
                            - Vect.read().normalize()
                                * if *infinite {
                                    expr
                                } else {
                                    expr.max(-Num.read().length()).min(Num.read().length())
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
                        Num.write(dist.clone().into()),
                        Distance.write(Distance.read() - Num.read()),
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
                        Num.write(Distance.read()),
                        Distance.write(Num.read().abs()),
                        Gradient.write(Gradient.read() * Num.read().sign()),
                    ]
                    .block(),
                )
            }
        },
        Elysian::Combine { .. } => unimplemented!(),
        Elysian::Alias(_) => panic!("Aliases must be expanded before conversion to Ast"),
    }
}
