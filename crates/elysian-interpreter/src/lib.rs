use std::{collections::BTreeMap, fmt::Debug, hash::Hasher};

use elysian_core::ir::{
    ast::Expr::*,
    ast::Stmt::{self, *},
    ast::{
        Identifier, Struct, StructIO, TypeSpec,
        Value::{self, *},
        VectorSpace, CONTEXT,
    },
    module::{FunctionDefinition, Module},
};
use rust_gpu_bridge::{Abs, Dot, Length, Max, Min, Mix, Normalize, Sign};

pub struct Interpreter<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub context: Struct<T, N>,
    pub functions: BTreeMap<Identifier, FunctionDefinition<T, N>>,
    pub output: Option<Value<T, N>>,
}

impl<T, const N: usize> Debug for Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interpreter")
            .field("context", &self.context)
            .field("functions", &self.functions)
            .field("output", &self.output)
            .finish()
    }
}

impl<T, const N: usize> Default for Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn default() -> Self {
        Self {
            context: Default::default(),
            functions: Default::default(),
            output: Default::default(),
        }
    }
}

impl<T, const N: usize> Clone for Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            functions: self.functions.clone(),
            output: self.output.clone(),
        }
    }
}

impl<T, const N: usize> std::hash::Hash for Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.hash(state);
        self.functions.hash(state);
        self.output.hash(state);
    }
}

pub fn evaluate_module<T, const N: usize>(
    mut interpreter: Interpreter<T, N>,
    module: &Module<T, N>,
) -> Struct<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    interpreter.context = Struct::default().set(CONTEXT, Value::Struct(interpreter.context));
    interpreter.functions = module
        .function_definitions
        .iter()
        .cloned()
        .map(|def| (def.id.clone(), def))
        .collect();

    let entry_point = module
        .function_definitions
        .iter()
        .find(|f| f.id == module.entry_point)
        .expect("No entry point");

    let interpreter = evaluate_block(interpreter, &entry_point.block);
    let Some(output) = interpreter.output else {
        panic!("No return value\n{:#?}", interpreter.context);
    };

    let Value::Struct(context) = output else {
        panic!("Invalid return value");
    };

    context
}

pub fn evaluate_stmt<T, const N: usize>(
    mut interpreter: Interpreter<T, N>,
    stmt: &Stmt<T, N>,
) -> Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
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

            innermost.set_mut(prop.clone(), v);

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
        _ => unimplemented!(),
    }
}

pub fn evaluate_block<T, const N: usize>(
    interpreter: Interpreter<T, N>,
    elysian_core::ir::ast::Block(list): &elysian_core::ir::ast::Block<T, N>,
) -> Interpreter<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    list.iter().fold(interpreter, evaluate_stmt)
}

pub fn evaluate_expr<T, const N: usize>(
    interpreter: &Interpreter<T, N>,
    expr: &elysian_core::ir::ast::Expr<T, N>,
) -> Value<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    match expr {
        Literal(l) => l.clone(),
        Read(expr, path) => path.iter().fold(
            if let Some(expr) = expr {
                evaluate_expr(interpreter, expr)
            } else {
                Struct(interpreter.context.clone())
            },
            |acc, next| match acc {
                Value::Struct(s) => s.get(next),
                v => v,
            },
        ),
        Construct(_, exprs) => {
            let mut s = Struct::default();
            for (prop, expr) in exprs {
                s.set_mut(prop.clone(), evaluate_expr(interpreter, expr));
            }
            Value::Struct(s)
        }
        Call { function, args } => {
            let f = interpreter
                .functions
                .get(function)
                .unwrap_or_else(|| panic!("Invalid function {:#?}", function));

            let context = Struct {
                members: f
                    .inputs
                    .iter()
                    .map(|input| input.prop.clone())
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
            (Vector2(lhs), Vector2(rhs)) => Vector2(lhs + rhs),
            (Number(lhs), Vector2(rhs)) => Vector2(lhs + rhs),
            (Vector2(lhs), Number(rhs)) => Vector2(lhs + rhs),
            _ => panic!("Invalid Add"),
        },
        Sub(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs - rhs),
            (Vector2(lhs), Vector2(rhs)) => Vector2(lhs - rhs),
            (Number(lhs), Vector2(rhs)) => Vector2(lhs - rhs),
            (Vector2(lhs), Number(rhs)) => Vector2(lhs - rhs),
            _ => panic!("Invalid Sub"),
        },
        Mul(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector2(lhs), Vector2(rhs)) => Vector2(lhs * rhs),
            (Number(lhs), Vector2(rhs)) => Vector2(lhs * rhs),
            (Vector2(lhs), Number(rhs)) => Vector2(lhs * rhs),
            _ => panic!("Invalid Mul"),
        },
        Div(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs / rhs),
            (Vector2(lhs), Vector2(rhs)) => Vector2(lhs / rhs),
            (Number(lhs), Vector2(rhs)) => Vector2(lhs / rhs),
            (Vector2(lhs), Number(rhs)) => Vector2(lhs / rhs),
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
            (Vector2(lhs), Vector2(rhs), Number(t)) => Vector2(lhs.mix(rhs, t)),
            _ => panic!("Invalid Mix"),
        },
        Neg(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(-n),
            Vector2(v) => Vector2(-v),
            _ => panic!("Invalid Neg"),
        },
        Abs(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.abs()),
            Vector2(v) => Vector2(v.abs()),
            _ => panic!("Invalid Abs"),
        },
        Sign(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.sign()),
            _ => panic!("Invalid Sign"),
        },
        Length(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n),
            Vector2(v) => Number(v.length()),
            _ => panic!("Invalid Length"),
        },
        Normalize(op) => match evaluate_expr(interpreter, op) {
            Number(n) => Number(n.sign()),
            Vector2(v) => Vector2(v.normalize()),
            _ => panic!("Invalid Normalize"),
        },
        Dot(lhs, rhs) => match (
            evaluate_expr(interpreter, lhs),
            evaluate_expr(interpreter, rhs),
        ) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector2(lhs), Vector2(rhs)) => Number(lhs.dot(rhs)),
            _ => panic!("Invalid Div"),
        },
        _ => unimplemented!(),
    }
}
