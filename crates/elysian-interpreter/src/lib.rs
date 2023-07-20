use std::{collections::BTreeMap, fmt::Debug, hash::Hasher};

use elysian_core::ir::{
    ast::Stmt::{self, *},
    ast::{
        Expr, Identifier, Struct,
        Value::{self, *},
        CONTEXT,
    },
    module::{FunctionDefinition, Module},
};
use rust_gpu_bridge::{Abs, Dot, Length, Max, Min, Mix, Normalize, Sign};

pub struct Interpreter {
    pub context: Struct,
    pub functions: BTreeMap<Identifier, FunctionDefinition>,
    pub output: Option<Value>,
}

impl Debug for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interpreter")
            .field("context", &self.context)
            .field("functions", &self.functions)
            .field("output", &self.output)
            .finish()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            context: Default::default(),
            functions: Default::default(),
            output: Default::default(),
        }
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            functions: self.functions.clone(),
            output: self.output.clone(),
        }
    }
}

impl std::hash::Hash for Interpreter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.hash(state);
        self.functions.hash(state);
        self.output.hash(state);
    }
}

pub fn evaluate_module(mut interpreter: Interpreter, module: &Module) -> Struct {
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

pub fn evaluate_stmt(mut interpreter: Interpreter, stmt: &Stmt) -> Interpreter {
    match stmt {
        Block(block) => evaluate_block(interpreter, block),
        Write { path, expr, .. } => {
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
        Loop { stmt } => {
            loop {
                match &**stmt {
                    Stmt::Break => break,
                    stmt => interpreter = evaluate_stmt(interpreter, &stmt),
                }
            }

            interpreter
        }
        Output(o) => {
            let o = evaluate_expr(&interpreter, o);
            interpreter.output = Some(o);
            interpreter
        }
        _ => unimplemented!(),
    }
}

pub fn evaluate_block(
    interpreter: Interpreter,
    elysian_core::ir::ast::Block(list): &elysian_core::ir::ast::Block,
) -> Interpreter {
    list.iter().fold(interpreter, evaluate_stmt)
}

pub fn evaluate_expr(interpreter: &Interpreter, expr: &elysian_core::ir::ast::Expr) -> Value {
    match expr {
        Expr::Literal(l) => l.clone(),
        Expr::Read(expr, path) => path.iter().fold(
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
        Expr::Struct(_, exprs) => {
            let mut s = Struct::default();
            for (prop, expr) in exprs {
                s.set_mut(prop.clone(), evaluate_expr(interpreter, expr));
            }
            Value::Struct(s)
        }
        Expr::Call { function, args } => {
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
        Expr::Add(lhs, rhs) => evaluate_expr(interpreter, lhs) + evaluate_expr(interpreter, rhs),
        Expr::Sub(lhs, rhs) => evaluate_expr(interpreter, lhs) - evaluate_expr(interpreter, rhs),
        Expr::Mul(lhs, rhs) => evaluate_expr(interpreter, lhs) * evaluate_expr(interpreter, rhs),
        Expr::Div(lhs, rhs) => evaluate_expr(interpreter, lhs) / evaluate_expr(interpreter, rhs),
        Expr::Lt(lhs, rhs) => {
            (evaluate_expr(interpreter, lhs) < evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Gt(lhs, rhs) => {
            (evaluate_expr(interpreter, lhs) > evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Min(lhs, rhs) => evaluate_expr(interpreter, lhs).min(evaluate_expr(interpreter, rhs)),
        Expr::Max(lhs, rhs) => evaluate_expr(interpreter, lhs).max(evaluate_expr(interpreter, rhs)),
        Expr::Mix(lhs, rhs, t) => evaluate_expr(interpreter, lhs)
            .mix(
                evaluate_expr(interpreter, rhs),
                evaluate_expr(interpreter, t),
            )
            .into(),
        Expr::Neg(op) => -evaluate_expr(interpreter, op),
        Expr::Abs(op) => evaluate_expr(interpreter, op).abs(),
        Expr::Sign(op) => evaluate_expr(interpreter, op).sign(),
        Expr::Length(op) => evaluate_expr(interpreter, op).length(),
        Expr::Normalize(op) => evaluate_expr(interpreter, op).normalize(),
        Expr::Dot(lhs, rhs) => evaluate_expr(interpreter, lhs).dot(evaluate_expr(interpreter, rhs)),
        _ => unimplemented!(),
    }
}
