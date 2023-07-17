use rust_gpu_bridge::{One, Two, Zero};
use std::fmt::Debug;
use tracing::instrument;

use crate::ast::Elysian;
use crate::ir::{
    ast::{
        Block, Expr, IntoRead, IntoValue, IntoWrite, Stmt, COLOR, COMBINE_CONTEXT, CONTEXT,
        DISTANCE, ERROR, GRADIENT, LEFT, LIGHT, NUM, OUT, POSITION, RIGHT, SUPPORT, TANGENT, TIME,
        UV, VECT,
    },
    module::{FieldDefinition, FunctionDefinition, InputDefinition, Module, StructDefinition},
};

use super::{as_ir::AsIR, ast::Identifier};

pub const CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
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

pub const COMBINE_CONTEXT_STRUCT: &'static StructDefinition = &StructDefinition {
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

#[instrument]
pub fn elysian_struct_definitions<N, V>(elysian: &Elysian<N, V>) -> Vec<StructDefinition>
where
    N: Debug,
    V: Debug,
{
    vec![CONTEXT_STRUCT.clone(), COMBINE_CONTEXT_STRUCT.clone()]
}

#[instrument]
pub fn elysian_module<N, V>(elysian: &Elysian<N, V>) -> Module<N, V>
where
    N: 'static + Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: 'static + Debug + Clone + IntoValue<N, V>,
{
    let mut functions = elysian_functions(elysian);
    functions.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));
    functions.dedup_by(|lhs, rhs| lhs.id == rhs.id);

    let struct_definitions = elysian_struct_definitions(elysian);
    let entry_point = FunctionDefinition {
        id: Identifier::new("shape", 523056258704924944),
        public: true,
        inputs: vec![InputDefinition {
            prop: CONTEXT,
            mutable: false,
        }],
        output: CONTEXT_STRUCT,
        block: elysian_entry_point(elysian),
    };
    Module {
        entry_point,
        struct_definitions,
        function_definitions: functions,
    }
}

#[instrument]
pub fn elysian_entry_point<N, V>(elysian: &Elysian<N, V>) -> Block<N, V>
where
    N: 'static + Debug + Clone + IntoValue<N, V>,
    V: 'static + Debug + Clone + IntoValue<N, V>,
{
    Block(match elysian {
        Elysian::Field {
            pre_modifiers,
            field,
            post_modifiers,
        } => pre_modifiers
            .iter()
            .map(|modifier| Stmt::Write {
                path: vec![CONTEXT],
                expr: modifier.expression(CONTEXT.read()),
            })
            .chain([Stmt::Write {
                path: vec![CONTEXT],
                expr: field.expression(CONTEXT.read()),
            }])
            .chain(post_modifiers.iter().map(|modifier| Stmt::Write {
                path: vec![CONTEXT],
                expr: modifier.expression(CONTEXT.read()),
            }))
            .chain(std::iter::once([CONTEXT].read().output()))
            .collect(),
        Elysian::Combine { combinator, shapes } => {
            let mut stmts = vec![];
            stmts.extend(elysian_entry_point_combine(combinator, shapes));
            stmts.push([LEFT].read().output());
            stmts
        }
    })
}

#[instrument]
pub fn elysian_entry_point_combine<N, V>(
    combinator: &Vec<Box<dyn AsIR<N, V>>>,
    shapes: &Vec<Elysian<N, V>>,
) -> Vec<Stmt<N, V>>
where
    N: 'static + Debug + Clone + IntoValue<N, V>,
    V: 'static + Debug + Clone + IntoValue<N, V>,
{
    let combinator = combinator_list_expr(combinator);

    let block = vec![];

    let (_, out) = shapes
        .iter()
        .enumerate()
        .fold((LEFT, block), |(side, mut acc), (i, next)| {
            match next {
                Elysian::Field {
                    pre_modifiers,
                    field,
                    post_modifiers,
                } => {
                    acc.push([side.clone()].write([CONTEXT].read()));

                    acc.extend(pre_modifiers.iter().map(|modifier| {
                        side.clone().write(modifier.expression(side.clone().read()))
                    }));

                    acc.push(side.clone().write(field.expression(side.clone().read())));

                    acc.extend(post_modifiers.iter().map(|modifier| {
                        side.clone().write(modifier.expression(side.clone().read()))
                    }));
                }
                Elysian::Combine { combinator, shapes } => {
                    acc.extend(elysian_entry_point_combine(combinator, shapes));
                }
            }

            if i > 0 {
                acc.extend([
                    COMBINE_CONTEXT.write(Expr::Construct(
                        COMBINE_CONTEXT_STRUCT,
                        [(LEFT, [LEFT].read()), (RIGHT, [RIGHT].read())].into(),
                    )),
                    COMBINE_CONTEXT.write(combinator.clone()),
                    LEFT.write([COMBINE_CONTEXT, OUT].read()),
                ]);
            }

            (RIGHT, acc)
        });

    out
}

#[instrument]
pub fn combinator_list_expr<'a, I: IntoIterator<Item = &'a Box<dyn AsIR<N, V>>>, N: 'a, V: 'a>(
    combinators: I,
) -> Expr<N, V>
where
    I: Debug,
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
{
    combinators
        .into_iter()
        .fold(COMBINE_CONTEXT.read(), |acc: Expr<N, V>, next| {
            let Expr::Call{ function, args } = next.expression(acc) else  {
                panic!("Combinator expression is not a CallResult")
            };

            Expr::Call { function, args }
        })
}

#[instrument]
pub fn elysian_functions<N, V>(elysian: &Elysian<N, V>) -> Vec<FunctionDefinition<N, V>>
where
    N: 'static + Debug + Clone + One + Two + Zero + IntoValue<N, V>,
    V: 'static + Debug + Clone + IntoValue<N, V>,
{
    match elysian {
        Elysian::Field {
            pre_modifiers,
            field,
            post_modifiers,
        } => pre_modifiers
            .iter()
            .flat_map(AsIR::functions)
            .chain(field.functions())
            .chain(post_modifiers.iter().flat_map(AsIR::functions))
            .collect(),
        Elysian::Combine { combinator, shapes } => combinator
            .iter()
            .flat_map(AsIR::functions)
            .chain(shapes.iter().map(elysian_functions).flatten())
            .collect(),
    }
}
