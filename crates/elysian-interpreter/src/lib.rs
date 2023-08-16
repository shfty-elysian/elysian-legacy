//! Evaluate Elysian IR at runtime

use std::{collections::BTreeMap, fmt::Debug, hash::Hasher};

use elysian_core::identifier::Identifier;
use elysian_ir::{
    ast::Stmt::{self, *},
    ast::{Expr, Struct, Value},
    module::{FunctionDefinition, FunctionIdentifier, Module, StructIdentifier, CONTEXT},
};
use elysian_math::{
    Abs, Acos, Asin, Atan, Atan2, Clamp, Cos, Dot, Length, Max, Min, Mix, Normalize, Round, Sign,
    Sin, Tan,
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

impl Interpreter {
    pub fn evaluate(mut self, module: &Module) -> Struct {
        #[cfg(feature = "print")]
        println!(
            "{}Module",
            std::iter::repeat("\n").take(200).collect::<String>()
        );

        self.context = Struct::new(StructIdentifier(INTERPRETER_CONTEXT))
            .set(CONTEXT.into(), Value::Struct(self.context));
        self.functions = module
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

        self = self.evaluate_block(&entry_point.block);
        let Some(output) = self.output else {
        panic!("No return value\n{:#?}", self.context);
    };

        let Value::Struct(context) = output else {
        panic!("Invalid return value");
    };

        context
    }

    fn evaluate_stmt(mut self, stmt: &Stmt) -> Interpreter {
        match stmt {
            Block(block) => {
                #[cfg(feature = "print")]
                println!("Block");
                self.evaluate_block(block)
            }
            Bind { prop, expr } => {
                #[cfg(feature = "print")]
                println!("Bind {}", prop.name());
                let v = self.evaluate_expr(expr);
                self.context.set_mut(prop.clone(), v);
                self
            }
            Write { path, expr } => {
                let v = self.evaluate_expr(expr);

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
                        .fold(&mut self.context, |acc, next| {
                            let Value::Struct(s) = acc.get_mut(next) else {
                            panic!("Path element is not a struct");
                        };

                            s
                        });

                innermost.set_mut(prop.clone(), v);

                self
            }
            If {
                cond,
                then,
                otherwise,
            } => {
                #[cfg(feature = "print")]
                println!("If");
                let Value::Boolean(b) = self.evaluate_expr(cond) else {
                panic!("Invalid If");
            };

                if b {
                    self.evaluate_stmt(then)
                } else {
                    if let Some(otherwise) = otherwise {
                        self.evaluate_stmt(otherwise)
                    } else {
                        self
                    }
                }
            }
            Loop { stmt } => {
                #[cfg(feature = "print")]
                println!("Loop");
                loop {
                    self = self.evaluate_stmt(&stmt);
                    if self.should_break {
                        self.should_break = false;
                        break;
                    }
                }

                self
            }
            Break => {
                #[cfg(feature = "print")]
                println!("Break");
                self.should_break = true;
                self
            }
            Output(o) => {
                #[cfg(feature = "print")]
                println!("Output");
                let o = self.evaluate_expr(o);
                self.output = Some(o);
                self
            }
        }
    }

    fn evaluate_block(self, elysian_ir::ast::Block(list): &elysian_ir::ast::Block) -> Interpreter {
        #[cfg(feature = "print")]
        println!("Block");

        list.iter().fold(self, Interpreter::evaluate_stmt)
    }

    fn evaluate_expr(&self, expr: &elysian_ir::ast::Expr) -> Value {
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
                path.iter()
                    .fold(Value::Struct(self.context.clone()), |acc, next| match acc {
                        Value::Struct(s) => s.get(next),
                        v => v,
                    })
            }
            Expr::Struct(def, exprs) => {
                #[cfg(feature = "print")]
                println!("Struct {:}", def.name());
                let mut s = Struct::new(def.clone());
                for (prop, expr) in exprs {
                    s.set_mut(prop.clone(), self.evaluate_expr(expr));
                }
                Value::Struct(s)
            }
            Expr::Call { function, args } => {
                #[cfg(feature = "print")]
                println!("Call {:}", function.name_unique());

                let f = self
                    .functions
                    .get(function)
                    .unwrap_or_else(|| panic!("Invalid function {:#?}", function));

                let context = Struct {
                    id: StructIdentifier(CALL_CONTEXT),
                    members: f
                        .inputs
                        .iter()
                        .map(|input| input.id.clone())
                        .zip(args.iter().map(|arg| self.evaluate_expr(arg)))
                        .collect(),
                };

                Interpreter {
                    context,
                    functions: self.functions.clone(),
                    should_break: Default::default(),
                    output: Default::default(),
                }
                .evaluate_block(&f.block)
                .output
                .expect("Function returned nothing")
            }
            Expr::Neg(op) => {
                #[cfg(feature = "print")]
                println!("Neg");
                -self.evaluate_expr(op)
            }
            Expr::Abs(op) => {
                #[cfg(feature = "print")]
                println!("Abs");
                self.evaluate_expr(op).abs()
            }
            Expr::Sign(op) => {
                #[cfg(feature = "print")]
                println!("Sign");
                self.evaluate_expr(op).sign()
            }
            Expr::Round(op) => {
                #[cfg(feature = "print")]
                println!("Round");
                self.evaluate_expr(op).round()
            }
            Expr::Sin(op) => {
                #[cfg(feature = "print")]
                println!("Sin");
                self.evaluate_expr(op).sin()
            }
            Expr::Cos(op) => {
                #[cfg(feature = "print")]
                println!("Cos");
                self.evaluate_expr(op).cos()
            }
            Expr::Tan(op) => {
                #[cfg(feature = "print")]
                println!("Tan");
                self.evaluate_expr(op).tan()
            }
            Expr::Asin(op) => {
                #[cfg(feature = "print")]
                println!("Asin");
                self.evaluate_expr(op).asin()
            }
            Expr::Acos(op) => {
                #[cfg(feature = "print")]
                println!("Acos");
                self.evaluate_expr(op).acos()
            }
            Expr::Atan(op) => {
                #[cfg(feature = "print")]
                println!("Atan");
                self.evaluate_expr(op).atan()
            }
            Expr::Length(op) => {
                #[cfg(feature = "print")]
                println!("Length");
                self.evaluate_expr(op).length()
            }
            Expr::Normalize(op) => {
                #[cfg(feature = "print")]
                println!("Normalize");
                self.evaluate_expr(op).normalize()
            }
            Expr::Add(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Add");
                self.evaluate_expr(lhs) + self.evaluate_expr(rhs)
            }
            Expr::Sub(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Sub");
                self.evaluate_expr(lhs) - self.evaluate_expr(rhs)
            }
            Expr::Mul(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Mul");
                self.evaluate_expr(lhs) * self.evaluate_expr(rhs)
            }
            Expr::Div(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Div");
                self.evaluate_expr(lhs) / self.evaluate_expr(rhs)
            }
            Expr::Mod(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Mod");
                self.evaluate_expr(lhs) % self.evaluate_expr(rhs)
            }
            Expr::Eq(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Eq");
                (self.evaluate_expr(lhs) == self.evaluate_expr(rhs)).into()
            }
            Expr::Ne(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Ne");
                (self.evaluate_expr(lhs) != self.evaluate_expr(rhs)).into()
            }
            Expr::Lt(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Lt");
                (self.evaluate_expr(lhs) < self.evaluate_expr(rhs)).into()
            }
            Expr::Gt(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Gt");
                (self.evaluate_expr(lhs) > self.evaluate_expr(rhs)).into()
            }
            Expr::And(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("And");
                (self.evaluate_expr(lhs) & self.evaluate_expr(rhs)).into()
            }
            Expr::Or(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Or");
                (self.evaluate_expr(lhs) | self.evaluate_expr(rhs)).into()
            }
            Expr::Min(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Min");
                self.evaluate_expr(lhs).min(self.evaluate_expr(rhs))
            }
            Expr::Max(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Max");
                self.evaluate_expr(lhs).max(self.evaluate_expr(rhs))
            }
            Expr::Mix(lhs, rhs, t) => {
                #[cfg(feature = "print")]
                println!("Mix");
                self.evaluate_expr(lhs)
                    .mix(self.evaluate_expr(rhs), self.evaluate_expr(t))
                    .into()
            }
            Expr::Clamp(t, min, max) => {
                #[cfg(feature = "print")]
                println!("Clamp");
                self.evaluate_expr(t)
                    .clamp(self.evaluate_expr(min), self.evaluate_expr(max))
                    .into()
            }
            Expr::Dot(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Dot");
                self.evaluate_expr(lhs).dot(self.evaluate_expr(rhs))
            }
            Expr::Atan2(lhs, rhs) => {
                #[cfg(feature = "print")]
                println!("Atan2");
                self.evaluate_expr(lhs).atan2(self.evaluate_expr(rhs))
            }
        }
    }
}
