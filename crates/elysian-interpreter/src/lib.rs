use std::{collections::BTreeMap, fmt::Debug, hash::Hasher};

use elysian_core::ir::{
    ast::Stmt::{self, *},
    ast::{Expr, Identifier, Struct, Value},
    module::{FunctionDefinition, FunctionIdentifier, Module, StructIdentifier, CONTEXT},
};
use rust_gpu_bridge::{
    Abs, Acos, Atan, Atan2, Clamp, Dot, Length, Max, Min, Mix, Normalize, Round, Sign,
};

pub struct Interpreter {
    pub context: Struct,
    pub functions: BTreeMap<FunctionIdentifier, FunctionDefinition>,
    pub should_break: bool,
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
            context: Struct::new(StructIdentifier(CONTEXT)),
            functions: Default::default(),
            should_break: Default::default(),
            output: Default::default(),
        }
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            functions: self.functions.clone(),
            should_break: Default::default(),
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

pub const INTERPRETER_CONTEXT: Identifier =
    Identifier::new("InterpreterContext", 1198218077110787867);

const CALL_CONTEXT: Identifier = Identifier::new("CallContext", 0);

pub fn evaluate_module(mut interpreter: Interpreter, module: &Module) -> Struct {
    #[cfg(feature = "print")]
    println!(
        "{}Module",
        std::iter::repeat("\n").take(200).collect::<String>()
    );

    interpreter.context = Struct::new(StructIdentifier(INTERPRETER_CONTEXT))
        .set(CONTEXT.into(), Value::Struct(interpreter.context));
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
        Block(block) => {
            #[cfg(feature = "print")]
            println!("Block");
            evaluate_block(interpreter, block)
        }
        Bind { prop, expr } => {
            #[cfg(feature = "print")]
            println!("Bind {}", prop.name());
            let v = evaluate_expr(&interpreter, expr);
            interpreter.context.set_mut(prop.clone(), v);
            interpreter
        }
        Write { path, expr } => {
            let v = evaluate_expr(&interpreter, expr);

            #[cfg(feature = "print")]
            println!(
                "Write {} to {}",
                v.to_string(),
                path.iter()
                    .map(|prop| prop.name().to_string() + &".")
                    .collect::<String>()
            );

            let prop = path.last().expect("Path is empty");

            let innermost =
                path.iter()
                    .take(path.len() - 1)
                    .fold(&mut interpreter.context, |acc, next| {
                        let Value::Struct(s) = acc.get_mut(next) else {
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
            #[cfg(feature = "print")]
            println!("If");
            let Value::Boolean(b) = evaluate_expr(&interpreter, cond) else {
                panic!("Invalid If");
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
            #[cfg(feature = "print")]
            println!("Loop");
            loop {
                interpreter = evaluate_stmt(interpreter, &stmt);
                if interpreter.should_break {
                    interpreter.should_break = false;
                    break;
                }
            }

            interpreter
        }
        Break => {
            #[cfg(feature = "print")]
            println!("Break");
            interpreter.should_break = true;
            interpreter
        }
        Output(o) => {
            #[cfg(feature = "print")]
            println!("Output");
            let o = evaluate_expr(&interpreter, o);
            interpreter.output = Some(o);
            interpreter
        }
    }
}

pub fn evaluate_block(
    interpreter: Interpreter,
    elysian_core::ir::ast::Block(list): &elysian_core::ir::ast::Block,
) -> Interpreter {
    #[cfg(feature = "print")]
    println!("Block");

    list.iter().fold(interpreter, evaluate_stmt)
}

pub fn evaluate_expr(interpreter: &Interpreter, expr: &elysian_core::ir::ast::Expr) -> Value {
    match expr {
        Expr::Literal(l) => {
            #[cfg(feature = "print")]
            println!("Literal {l:#?}");
            l.clone()
        }
        Expr::Read(path) => {
            #[cfg(feature = "print")]
            println!(
                "Read {}",
                path.iter()
                    .map(|segment| segment.name().to_string() + &".")
                    .collect::<String>()
            );
            path.iter().fold(
                Value::Struct(interpreter.context.clone()),
                |acc, next| match acc {
                    Value::Struct(s) => s.get(next),
                    v => v,
                },
            )
        }
        Expr::Struct(def, exprs) => {
            #[cfg(feature = "print")]
            println!("Struct {:}", def.name());
            let mut s = Struct::new(def.clone());
            for (prop, expr) in exprs {
                s.set_mut(prop.clone(), evaluate_expr(interpreter, expr));
            }
            Value::Struct(s)
        }
        Expr::Call { function, args } => {
            #[cfg(feature = "print")]
            println!("Call {:}", function.name_unique());

            let f = interpreter
                .functions
                .get(function)
                .unwrap_or_else(|| panic!("Invalid function {:#?}", function));

            let context = Struct {
                id: StructIdentifier(CALL_CONTEXT),
                members: f
                    .inputs
                    .iter()
                    .map(|input| input.id.clone())
                    .zip(args.iter().map(|arg| evaluate_expr(interpreter, arg)))
                    .collect(),
            };

            evaluate_block(
                Interpreter {
                    context,
                    functions: interpreter.functions.clone(),
                    should_break: Default::default(),
                    output: Default::default(),
                },
                &f.block,
            )
            .output
            .expect("Function returned nothing")
        }
        Expr::Neg(op) => {
            #[cfg(feature = "print")]
            println!("Neg");
            -evaluate_expr(interpreter, op)
        }
        Expr::Abs(op) => {
            #[cfg(feature = "print")]
            println!("Abs");
            evaluate_expr(interpreter, op).abs()
        }
        Expr::Sign(op) => {
            #[cfg(feature = "print")]
            println!("Sign");
            evaluate_expr(interpreter, op).sign()
        }
        Expr::Round(op) => {
            #[cfg(feature = "print")]
            println!("Round");
            evaluate_expr(interpreter, op).round()
        }
        Expr::Acos(op) => {
            #[cfg(feature = "print")]
            println!("Acos");
            evaluate_expr(interpreter, op).acos()
        }
        Expr::Atan(op) => {
            #[cfg(feature = "print")]
            println!("Atan");
            evaluate_expr(interpreter, op).atan()
        }
        Expr::Length(op) => {
            #[cfg(feature = "print")]
            println!("Length");
            evaluate_expr(interpreter, op).length()
        }
        Expr::Normalize(op) => {
            #[cfg(feature = "print")]
            println!("Normalize");
            evaluate_expr(interpreter, op).normalize()
        }
        Expr::Add(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Add");
            evaluate_expr(interpreter, lhs) + evaluate_expr(interpreter, rhs)
        }
        Expr::Sub(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Sub");
            evaluate_expr(interpreter, lhs) - evaluate_expr(interpreter, rhs)
        }
        Expr::Mul(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Mul");
            evaluate_expr(interpreter, lhs) * evaluate_expr(interpreter, rhs)
        }
        Expr::Div(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Div");
            evaluate_expr(interpreter, lhs) / evaluate_expr(interpreter, rhs)
        }
        Expr::Mod(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Mod");
            evaluate_expr(interpreter, lhs) % evaluate_expr(interpreter, rhs)
        }
        Expr::Eq(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Eq");
            (evaluate_expr(interpreter, lhs) == evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Ne(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Ne");
            (evaluate_expr(interpreter, lhs) != evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Lt(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Lt");
            (evaluate_expr(interpreter, lhs) < evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Gt(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Gt");
            (evaluate_expr(interpreter, lhs) > evaluate_expr(interpreter, rhs)).into()
        }
        Expr::And(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("And");
            (evaluate_expr(interpreter, lhs) & evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Or(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Or");
            (evaluate_expr(interpreter, lhs) | evaluate_expr(interpreter, rhs)).into()
        }
        Expr::Min(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Min");
            evaluate_expr(interpreter, lhs).min(evaluate_expr(interpreter, rhs))
        }
        Expr::Max(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Max");
            evaluate_expr(interpreter, lhs).max(evaluate_expr(interpreter, rhs))
        }
        Expr::Mix(lhs, rhs, t) => {
            #[cfg(feature = "print")]
            println!("Mix");
            evaluate_expr(interpreter, lhs)
                .mix(
                    evaluate_expr(interpreter, rhs),
                    evaluate_expr(interpreter, t),
                )
                .into()
        }
        Expr::Clamp(t, min, max) => {
            #[cfg(feature = "print")]
            println!("Clamp");
            evaluate_expr(interpreter, t)
                .clamp(
                    evaluate_expr(interpreter, min),
                    evaluate_expr(interpreter, max),
                )
                .into()
        }
        Expr::Dot(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Dot");
            evaluate_expr(interpreter, lhs).dot(evaluate_expr(interpreter, rhs))
        }
        Expr::Atan2(lhs, rhs) => {
            #[cfg(feature = "print")]
            println!("Atan2");
            evaluate_expr(interpreter, lhs).atan2(evaluate_expr(interpreter, rhs))
        }
    }
}
