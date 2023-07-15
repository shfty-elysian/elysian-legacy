use std::{collections::BTreeMap, fmt::Debug};

use crate::ir::{
    ast::Expr::*,
    ast::Stmt::{self, *},
    ast::{
        Identifier, Number, Struct,
        Value::{self, *},
        Vector, CONTEXT,
    },
    module::{FunctionDefinition, Module},
};

#[derive(Debug, Default, Clone)]
pub struct Interpreter<N, V> {
    pub context: Struct<N, V>,
    pub functions: BTreeMap<Identifier, FunctionDefinition<N, V>>,
    pub output: Option<Value<N, V>>,
}

pub fn evaluate_module<N, V>(
    mut interpreter: Interpreter<N, V>,
    module: &Module<N, V>,
) -> Struct<N, V>
where
    N: Debug + Number<N, V>,
    V: Debug + Vector<N, V>,
{
    interpreter.context = Struct::default().set(CONTEXT, Value::Struct(interpreter.context));
    interpreter.functions = module
        .function_definitions
        .iter()
        .cloned()
        .map(|def| (def.id, def))
        .collect();

    let Some(output) = evaluate_block(interpreter, &module.entry_point.block).output else {
        panic!("No return value");
    };

    let Value::Struct(context) = output else {
        panic!("Invalid return value");
    };

    context
}

pub fn evaluate_stmt<N, V>(
    mut interpreter: Interpreter<N, V>,
    stmt: &Stmt<N, V>,
) -> Interpreter<N, V>
where
    N: Debug + Number<N, V>,
    V: Debug + Vector<N, V>,
{
    match stmt {
        Block(block) => evaluate_block(interpreter, block),
        Write { path, expr } => {
            let v = evaluate_expr(&interpreter, expr);

            let prop = path.last().expect("Path is empty");

            let innermost =
                path.iter()
                    .take(path.len() - 1)
                    .fold(&mut interpreter.context, |acc, next| {
                        let Struct(s) = acc.get_mut(next) else {
                            panic!("Path element is not a struct");
                        };

                        s
                    });

            innermost.set_mut(*prop, v);

            interpreter
        }
        If {
            cond,
            then,
            otherwise,
        } => {
            let Value::Boolean(b) = evaluate_expr(&interpreter, cond) else {
                    panic!("Invalid IfElse");
                };

            if b {
                evaluate_stmt(interpreter, then)
            } else {
                if let Some(otherwise) = otherwise {
                    evaluate_stmt(interpreter, otherwise)
                } else {
                    interpreter
                }
            }
        }
        Output(o) => {
            let o = evaluate_expr(&interpreter, o);
            interpreter.output = Some(o);
            interpreter
        }
    }
}

pub fn evaluate_block<N, V>(
    this: Interpreter<N, V>,
    crate::ir::ast::Block(list): &crate::ir::ast::Block<N, V>,
) -> Interpreter<N, V>
where
    N: Debug + Number<N, V>,
    V: Debug + Vector<N, V>,
{
    list.iter().fold(this, |acc, next| evaluate_stmt(acc, next))
}

pub fn evaluate_expr<N, V>(
    interpreter: &Interpreter<N, V>,
    expr: &crate::ir::ast::Expr<N, V>,
) -> Value<N, V>
where
    N: Debug + Number<N, V>,
    V: Debug + Vector<N, V>,
{
    match expr {
        Literal(l) => l.clone(),
        Read(path) => path
            .iter()
            .fold(Struct(interpreter.context.clone()), |acc, next| match acc {
                Value::Struct(s) => s.get(next),
                v => v,
            })
            .clone(),
        Construct(_, exprs) => {
            let mut s = Struct::default();
            for (prop, expr) in exprs {
                s.set_mut(*prop, evaluate_expr(interpreter, expr));
            }
            Value::Struct(s)
        }
        Call { function, args } => {
            let f = interpreter
                .functions
                .get(function)
                .expect("Invalid function");

            let context = Struct {
                members: f
                    .inputs
                    .iter()
                    .map(|input| input.prop)
                    .zip(args.iter().map(|arg| evaluate_expr(interpreter, arg)))
                    .collect(),
            };

            evaluate_block(
                Interpreter {
                    context,
                    functions: interpreter.functions.clone(),
                    output: None,
                },
                &f.block,
            )
            .output
            .expect("Function returned nothing")
        }
        Add(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs + rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs + rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs + rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs + rhs),
            _ => panic!("Invalid Add"),
        },
        Sub(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs - rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs - rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs - rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs - rhs),
            _ => panic!("Invalid Sub"),
        },
        Mul(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs * rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs * rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs * rhs),
            _ => panic!("Invalid Mul"),
        },
        Div(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs / rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs / rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs / rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs / rhs),
            _ => panic!("Invalid Div"),
        },
        Lt(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Boolean(lhs < rhs),
            _ => panic!("Invalid Lt"),
        },
        Gt(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Boolean(lhs > rhs),
            _ => panic!("Invalid Gt"),
        },
        Min(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs.min(rhs)),
            _ => panic!("Invalid Min"),
        },
        Max(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs.max(rhs)),
            _ => panic!("Invalid Max"),
        },
        Mix(lhs, rhs, t) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
            evaluate_expr(interpreter, t),
        ) {
            (Number(lhs), Number(rhs), Number(t)) => Number(lhs.mix(rhs, t)),
            (Vector(lhs), Vector(rhs), Number(t)) => Vector(lhs.mix(rhs, t)),
            _ => panic!("Invalid Mix"),
        },
        Neg(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(-n),
            Vector(v) => Vector(-v),
            _ => panic!("Invalid Neg"),
        },
        Abs(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.abs()),
            Vector(v) => Vector(v.abs()),
            _ => panic!("Invalid Abs"),
        },
        Sign(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.sign()),
            _ => panic!("Invalid Sign"),
        },
        Length(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n),
            Vector(v) => Number(v.length()),
            _ => panic!("Invalid Length"),
        },
        Normalize(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.sign()),
            Vector(v) => Vector(v.normalize()),
            _ => panic!("Invalid Normalize"),
        },
        Dot(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector(lhs), Vector(rhs)) => Number(lhs.dot(rhs)),
            _ => panic!("Invalid Div"),
        },
    }
}
