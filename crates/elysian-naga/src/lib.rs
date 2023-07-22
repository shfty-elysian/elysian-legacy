use std::collections::BTreeMap;

use elysian_core::ir::{
    ast::{Expr, Number, Property, Stmt, Value},
    module::{AsModule, FunctionDefinition, SpecializationData, StructDefinition},
};
use naga::{
    Arena, BinaryOperator, Block as NagaBlock, Expression, Function, FunctionArgument,
    FunctionResult, Handle, Literal, LocalVariable, MathFunction, Range, ScalarKind, Span,
    Statement, StructMember, Type, TypeInner, UniqueArena, VectorSize,
};

pub fn module_to_naga<T>(input: &T, spec: &SpecializationData, name: &str) -> naga::Module
where
    T: AsModule,
{
    let module = input.module(spec);

    let types = structs_to_naga(&module.struct_definitions);
    let functions = functions_to_naga(&module.function_definitions, &types);

    let out = naga::Module {
        types,
        special_types: Default::default(),
        constants: Default::default(),
        global_variables: Default::default(),
        const_expressions: Default::default(),
        functions,
        entry_points: Default::default(),
    };

    out
}

fn structs_to_naga(defs: &Vec<StructDefinition>) -> UniqueArena<Type> {
    let mut types = UniqueArena::default();

    let bool = types.insert(
        Type {
            name: Some("Bool".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Bool,
                width: 1,
            },
        },
        Span::UNDEFINED,
    );

    let float = types.insert(
        Type {
            name: Some("f32".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Float,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let vec2 = types.insert(
        Type {
            name: Some("Vector2".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Bi,
                kind: ScalarKind::Float,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let vec3 = types.insert(
        Type {
            name: Some("Vector3".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Tri,
                kind: ScalarKind::Float,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let vec4 = types.insert(
        Type {
            name: Some("Vector4".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Quad,
                kind: ScalarKind::Float,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let mat2 = types.insert(
        Type {
            name: Some("Matrix2".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Bi,
                rows: VectorSize::Bi,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let mat3 = types.insert(
        Type {
            name: Some("Matrix3".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Tri,
                rows: VectorSize::Tri,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    let mat4 = types.insert(
        Type {
            name: Some("Matrix4".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Quad,
                rows: VectorSize::Quad,
                width: 4,
            },
        },
        Span::UNDEFINED,
    );

    for def in defs {
        let (members, span) =
            def.fields
                .iter()
                .fold((vec![], 0), |(mut members, total_span), next| {
                    let (member, span) = match next.prop.ty() {
                        elysian_core::ir::module::Type::Boolean => (bool, 1),
                        elysian_core::ir::module::Type::Number => (float, 4),
                        elysian_core::ir::module::Type::Vector2 => (vec2, 8),
                        elysian_core::ir::module::Type::Vector3 => (vec3, 12),
                        elysian_core::ir::module::Type::Vector4 => (vec4, 16),
                        elysian_core::ir::module::Type::Matrix2 => (mat2, 24),
                        elysian_core::ir::module::Type::Matrix3 => (mat3, 36),
                        elysian_core::ir::module::Type::Matrix4 => (mat4, 48),
                        elysian_core::ir::module::Type::Struct(s) => {
                            let (handle, ty) = types
                                .iter()
                                .find(|(_, v)| v.name == Some(s.id.name().to_string()))
                                .unwrap();
                            let TypeInner::Struct {
                                span,
                                ..
                            } = ty.inner else {
                                panic!("Type is not a Struct");
                            };
                            (handle, span)
                        }
                    };
                    members.push(StructMember {
                        name: Some(next.prop.name().to_string()),
                        ty: member,
                        binding: None,
                        offset: total_span,
                    });
                    (members, total_span + span)
                });

        let ty = Type {
            name: Some(def.name().to_string()),
            inner: TypeInner::Struct { members, span },
        };

        types.insert(ty, Span::UNDEFINED);
    }

    types
}

fn functions_to_naga(defs: &Vec<FunctionDefinition>, tys: &UniqueArena<Type>) -> Arena<Function> {
    let mut functions = Arena::default();

    let handles = defs
        .iter()
        .map(|def| {
            (
                functions.append(
                    Function {
                        name: Some(def.name().to_string()),
                        arguments: def
                            .inputs
                            .iter()
                            .map(|input| FunctionArgument {
                                name: Some(input.prop.name().to_string()),
                                ty: tys
                                    .iter()
                                    .find(|(_, v)| v.name == Some(def.output.name().to_string()))
                                    .expect("No type")
                                    .0,
                                binding: None,
                            })
                            .collect(),
                        result: Some(FunctionResult {
                            ty: tys
                                .iter()
                                .find(|(_, v)| v.name == Some(def.output.name().to_string()))
                                .expect("No type")
                                .0,
                            binding: None,
                        }),
                        local_variables: Default::default(),
                        expressions: Default::default(),
                        named_expressions: Default::default(),
                        body: Default::default(),
                    },
                    Span::UNDEFINED,
                ),
                def,
            )
        })
        .collect::<BTreeMap<_, _>>();

    for (handle, def) in handles {
        let mut body = NagaBlock::new();
        let mut expressions = Arena::default();
        let mut local_variables = Arena::default();

        for stmt in def.block.0.iter() {
            stmt_to_naga(
                stmt,
                tys,
                &functions,
                &mut body,
                &mut expressions,
                &mut local_variables,
            );
        }

        let f = functions.get_mut(handle);
        f.expressions = expressions;
        f.local_variables = local_variables;
        f.body = body;
    }

    functions
}

fn access_index(base: Handle<Expression>, prev: &Property, next: &Property) -> Expression {
    Expression::AccessIndex {
        base,
        index: match prev.ty() {
            elysian_core::ir::module::Type::Vector2 => match next.name() {
                "x" => 0,
                "y" => 1,
                _ => panic!("Invalid Vector2 access"),
            },
            elysian_core::ir::module::Type::Vector3 => match next.name() {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                t => panic!("Invalid Vector3 access {t:#?}"),
            },
            elysian_core::ir::module::Type::Vector4 => match next.name() {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                "w" => 3,
                _ => panic!("Invalid Vector4 access"),
            },
            elysian_core::ir::module::Type::Matrix2 => match next.name() {
                "x" => 0,
                "y" => 1,
                _ => panic!("Invalid Matrix2 access"),
            },
            elysian_core::ir::module::Type::Matrix3 => match next.name() {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                _ => panic!("Invalid Matrix3 access"),
            },
            elysian_core::ir::module::Type::Matrix4 => match next.name() {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                "w" => 3,
                _ => panic!("Invalid Matrix4 access"),
            },
            elysian_core::ir::module::Type::Struct(s) => s
                .fields
                .iter()
                .position(|field| field.prop == *next)
                .unwrap_or_else(|| panic!("No field {next:#?} for struct {s:#?}"))
                as u32,
            t => panic!("Not a struct: {t:#?}"),
        },
    }
}

fn stmt_to_naga(
    stmt: &Stmt,
    tys: &UniqueArena<Type>,
    functions: &Arena<Function>,
    body: &mut NagaBlock,
    expressions: &mut Arena<Expression>,
    local_variables: &mut Arena<LocalVariable>,
) {
    match stmt {
        Stmt::Block(block) => block
            .0
            .iter()
            .map(|t| stmt_to_naga(t, tys, functions, body, expressions, local_variables))
            .collect(),
        Stmt::Bind { prop, expr } => {
            let pointer = expressions.append(
                naga::Expression::LocalVariable(
                    local_variables.append(
                        naga::LocalVariable {
                            name: Some(prop.name().to_string()),
                            ty: tys
                                .iter()
                                .find(|(_, v)| v.name == Some(prop.ty().name().to_string()))
                                .unwrap_or_else(|| {
                                    panic!("No Type for {}", prop.ty().name())
                                })
                                .0,
                            init: None,
                        },
                        Span::UNDEFINED,
                    ),
                ),
                Span::UNDEFINED,
            );

            let value = expr_to_naga(expr, tys, functions, body, local_variables, expressions);
            body.push(
                Statement::Emit(Range::new_from_bounds(value, value)),
                Span::UNDEFINED,
            );

            body.push(Statement::Store { pointer, value }, Span::UNDEFINED)
        }
        Stmt::Write { path, expr } => {
            let mut iter = path.iter();

            let base = iter.next().unwrap();

            let base_expr = if let Some(k) = local_variables
                .iter()
                .find(|(_, v)| v.name == Some(base.name().to_string()))
                .map(|(k, _)| k)
            {
                let base_expr = expressions.append(Expression::LocalVariable(k), Span::UNDEFINED);
                body.push(
                    Statement::Emit(Range::new_from_bounds(base_expr, base_expr)),
                    Span::UNDEFINED,
                );
                base_expr
            } else {
                let base_expr =
                    expressions.append(Expression::FunctionArgument(0), Span::UNDEFINED);
                base_expr
            };

            let (_, pointer) = iter.fold((base.clone(), base_expr), |(prev, expr), next| {
                (
                    next.clone(),
                    expressions.append(access_index(expr, &prev, next), Span::UNDEFINED),
                )
            });

            let value = expr_to_naga(expr, tys, functions, body, local_variables, expressions);

            body.push(
                Statement::Emit(Range::new_from_bounds(value, value)),
                Span::UNDEFINED,
            );

            body.push(Statement::Store { pointer, value }, Span::UNDEFINED)
        }
        Stmt::If {
            cond,
            then,
            otherwise,
        } => {
            let condition = expr_to_naga(cond, tys, functions, body, local_variables, expressions);

            let mut accept = NagaBlock::default();
            stmt_to_naga(
                then,
                tys,
                functions,
                &mut accept,
                expressions,
                local_variables,
            );

            let mut reject = NagaBlock::default();

            if let Some(otherwise) = otherwise {
                stmt_to_naga(
                    otherwise,
                    tys,
                    functions,
                    &mut reject,
                    expressions,
                    local_variables,
                )
            };

            body.push(
                Statement::Emit(Range::new_from_bounds(condition, condition)),
                Span::UNDEFINED,
            );

            body.push(
                Statement::If {
                    condition,
                    accept,
                    reject,
                },
                Span::UNDEFINED,
            )
        }
        Stmt::Loop { stmt } => {
            let mut loop_body = NagaBlock::default();
            stmt_to_naga(
                stmt,
                tys,
                functions,
                &mut loop_body,
                expressions,
                local_variables,
            );

            body.push(
                Statement::Loop {
                    body: loop_body,
                    continuing: Default::default(),
                    break_if: None,
                },
                Span::UNDEFINED,
            )
        }
        Stmt::Break => body.push(Statement::Break, Span::UNDEFINED),
        Stmt::Output(expr) => {
            let value = expr_to_naga(expr, tys, functions, body, local_variables, expressions);
            body.push(
                Statement::Emit(Range::new_from_bounds(value, value)),
                Span::UNDEFINED,
            );
            body.push(Statement::Return { value: Some(value) }, Span::UNDEFINED);
        }
    }
}

fn expr_to_naga(
    expr: &Expr,
    tys: &UniqueArena<Type>,
    functions: &Arena<Function>,
    body: &mut NagaBlock,
    local_variables: &Arena<LocalVariable>,
    expressions: &mut Arena<Expression>,
) -> Handle<Expression> {
    match expr {
        Expr::Literal(v) => {
            let value = value_to_naga(v, tys, expressions);
            expressions.append(value, Span::UNDEFINED)
        }
        Expr::Struct(def, members) => {
            let components = members
                .into_iter()
                .map(|(_, member)| {
                    expr_to_naga(member, tys, functions, body, local_variables, expressions)
                })
                .collect();
            expressions.append(
                Expression::Compose {
                    ty: tys
                        .iter()
                        .find(|(_, v)| v.name == Some(def.name().to_string()))
                        .unwrap()
                        .0,
                    components,
                },
                Span::UNDEFINED,
            )
        }
        Expr::Read(path) => {
            let mut iter = path.iter();

            let base = iter.next().unwrap();

            let base_expr = if let Some(k) = local_variables
                .iter()
                .find(|(_, v)| v.name == Some(base.name().to_string()))
                .map(|(k, _)| k)
            {
                let base_expr = expressions.append(Expression::LocalVariable(k), Span::UNDEFINED);
                body.push(
                    Statement::Emit(Range::new_from_bounds(base_expr, base_expr)),
                    Span::UNDEFINED,
                );
                base_expr
            } else {
                let base_expr =
                    expressions.append(Expression::FunctionArgument(0), Span::UNDEFINED);
                base_expr
            };

            let (_, exprs) = iter.fold((base.clone(), base_expr), |(prev, expr), next| {
                (
                    next.clone(),
                    expressions.append(access_index(expr, &prev, next), Span::UNDEFINED),
                )
            });

            exprs
        }
        Expr::Call { function, args } => {
            let function = functions
                .iter()
                .find(|(_, v)| v.name == Some(function.name().to_string()))
                .unwrap()
                .0;

            let arguments = args
                .into_iter()
                .map(|arg| expr_to_naga(arg, tys, functions, body, local_variables, expressions))
                .collect();

            let expr = expressions.append(Expression::CallResult(function), Span::UNDEFINED);

            body.push(
                Statement::Call {
                    function,
                    arguments,
                    result: Some(expr.clone()),
                },
                Span::UNDEFINED,
            );

            expr
        }
        Expr::Neg(t) => {
            let expr = expr_to_naga(t, tys, functions, body, local_variables, expressions);
            expressions.append(
                Expression::Unary {
                    op: naga::UnaryOperator::Negate,
                    expr,
                },
                Span::UNDEFINED,
            )
        }
        Expr::Add(lhs, rhs)
        | Expr::Sub(lhs, rhs)
        | Expr::Mul(lhs, rhs)
        | Expr::Div(lhs, rhs)
        | Expr::Lt(lhs, rhs)
        | Expr::Gt(lhs, rhs) => {
            let left = expr_to_naga(lhs, tys, functions, body, local_variables, expressions);
            let right = expr_to_naga(rhs, tys, functions, body, local_variables, expressions);
            expressions.append(
                Expression::Binary {
                    op: match expr {
                        Expr::Add(..) => BinaryOperator::Add,
                        Expr::Sub(..) => BinaryOperator::Subtract,
                        Expr::Mul(..) => BinaryOperator::Multiply,
                        Expr::Div(..) => BinaryOperator::Divide,
                        Expr::Lt(..) => BinaryOperator::Less,
                        Expr::Gt(..) => BinaryOperator::Greater,
                        _ => unreachable!(),
                    },
                    left,
                    right,
                },
                Span::UNDEFINED,
            )
        }
        Expr::Min(lhs, rhs) | Expr::Max(lhs, rhs) | Expr::Dot(lhs, rhs) => {
            let arg = expr_to_naga(lhs, tys, functions, body, local_variables, expressions);
            let arg1 = expr_to_naga(rhs, tys, functions, body, local_variables, expressions);
            expressions.append(
                Expression::Math {
                    fun: match expr {
                        Expr::Min(..) => MathFunction::Min,
                        Expr::Max(..) => MathFunction::Max,
                        Expr::Dot(..) => MathFunction::Dot,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: Some(arg1),
                    arg2: None,
                    arg3: None,
                },
                Span::UNDEFINED,
            )
        }
        Expr::Abs(t) | Expr::Sign(t) | Expr::Length(t) | Expr::Normalize(t) => {
            let arg = expr_to_naga(t, tys, functions, body, local_variables, expressions);
            expressions.append(
                Expression::Math {
                    fun: match expr {
                        Expr::Abs(..) => MathFunction::Abs,
                        Expr::Sign(..) => MathFunction::Sign,
                        Expr::Length(..) => MathFunction::Length,
                        Expr::Normalize(..) => MathFunction::Normalize,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: None,
                    arg2: None,
                    arg3: None,
                },
                Span::UNDEFINED,
            )
        }
        Expr::Mix(lhs, rhs, t) => {
            let arg = expr_to_naga(lhs, tys, functions, body, local_variables, expressions);
            let arg1 = expr_to_naga(rhs, tys, functions, body, local_variables, expressions);
            let arg2 = expr_to_naga(t, tys, functions, body, local_variables, expressions);
            expressions.append(
                Expression::Math {
                    fun: MathFunction::Mix,
                    arg,
                    arg1: Some(arg1),
                    arg2: Some(arg2),
                    arg3: None,
                },
                Span::UNDEFINED,
            )
        }
    }
}

fn number_to_naga(number: &Number) -> Expression {
    match number {
        Number::UInt(u) => Expression::Literal(Literal::U32(*u as u32)),
        Number::SInt(i) => Expression::Literal(Literal::I32(*i as i32)),
        Number::Float(f) => Expression::Literal(Literal::F32(*f as f32)),
    }
}

fn value_to_naga(
    value: &Value,
    tys: &UniqueArena<Type>,
    expressions: &mut Arena<Expression>,
) -> Expression {
    match value {
        Value::Boolean(b) => Expression::Literal(Literal::Bool(*b)),
        Value::Number(n) => number_to_naga(n),
        Value::Struct(s) => {
            let ty = tys
                .iter()
                .find(|(_, v)| v.name == Some(s.def.name().to_string()))
                .unwrap()
                .0;

            let mut components = vec![];

            for field in s.def.fields {
                let v = s.get(&field.prop);
                let v = value_to_naga(&v, tys, expressions);
                let v = expressions.append(v, Span::UNDEFINED);
                components.push(v);
            }

            Expression::Compose { ty, components }
        }
    }
}
