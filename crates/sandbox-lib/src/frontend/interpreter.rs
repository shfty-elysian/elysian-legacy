use std::fmt::Debug;

use crate::ir::{
    ast::Expr::*,
    ast::Stmt::{*, self},
    module::Module,
    ast::{
        Number, Struct,
        Value::{self, *},
        Vector,
    },
};

#[derive(Debug, Default, Clone)]
pub struct Interpreter<N, V> {
    pub context: Struct<N, V>,
}

pub fn evaluate_module<N, V>(this: Interpreter<N, V>, module: &Module<N, V>) -> Interpreter<N, V>
where
    N: Number<N, V>,
    V: Vector<N, V>,
{
    evaluate_stmt(this, &Block(module.entry_point.block.clone()))
}

pub fn evaluate_stmt<N, V>(
    mut this: Interpreter<N, V>,
    stmt: &Stmt<N, V>,
) -> Interpreter<N, V>
where
    N: Number<N, V>,
    V: Vector<N, V>,
{
    match stmt {
        Block(block) => evaluate_block(this, block),
        /*
        Fold(combinator, list) => {
            let mut iter = list.0.iter();

            let Some(ast) = iter.next() else {
                    return this;
                };

            let context = iter.fold(evaluate_stmt(this.clone(), ast).context, |ctx_a, ast| {
                let ctx_b = evaluate_stmt(this.clone(), ast).context;
                let ctx = Struct::default()
                    .set(Left, Struct(ctx_a))
                    .set(Right, Struct(ctx_b));

                let mut ctx = evaluate_block(Interpreter { context: ctx }, combinator).context;

                let Struct(c) = ctx.remove(&Out) else {
                        panic!("No output")
                    };

                c
            });

            Interpreter { context }
        }
        */
        Write { path, expr } => {
            let v = evaluate_expr(&this, expr);

            let prop = path.last().expect("Path is empty");

            let innermost =
                path.iter()
                    .take(path.len() - 1)
                    .fold(&mut this.context, |acc, next| {
                        let Struct(s) = acc.get_mut(next) else {
                            panic!("Path element is not a struct");
                        };

                        s
                    });

            innermost.set_mut(*prop, v);

            this
        }
        IfElse {
            cond,
            then,
            otherwise,
        } => {
            let Value::Boolean(b) = evaluate_expr(&this, cond) else {
                    panic!("Invalid IfElse");
                };

            if b {
                evaluate_stmt(this, then)
            } else {
                evaluate_stmt(this, otherwise)
            }
        }
        Nop => this,
        Output(_) => this,
    }
}

pub fn evaluate_block<N, V>(
    this: Interpreter<N, V>,
    crate::ir::ast::Block(list): &crate::ir::ast::Block<N, V>,
) -> Interpreter<N, V>
where
    N: Number<N, V>,
    V: Vector<N, V>,
{
    list.iter().fold(this, |acc, next| evaluate_stmt(acc, next))
}

pub fn evaluate_expr<N, V>(
    this: &Interpreter<N, V>,
    expr: &crate::ir::ast::Expr<N, V>,
) -> Value<N, V>
where
    N: Number<N, V>,
    V: Vector<N, V>,
{
    match expr {
        Literal(l) => l.clone(),
        Read(path) => path
            .iter()
            .fold(Struct(this.context.clone()), |acc, next| match acc {
                Value::Struct(s) => s.get(next),
                v => v,
            })
            .clone(),
        Construct(structure, exprs) => todo!(),
        Call {
            function,
            args: arg,
        } => todo!(),
        Add(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs + rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs + rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs + rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs + rhs),
            _ => panic!("Invalid Add"),
        },
        Sub(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs - rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs - rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs - rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs - rhs),
            _ => panic!("Invalid Sub"),
        },
        Mul(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs * rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs * rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs * rhs),
            _ => panic!("Invalid Mul"),
        },
        Div(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs / rhs),
            (Vector(lhs), Vector(rhs)) => Vector(lhs / rhs),
            (Number(lhs), Vector(rhs)) => Vector(lhs / rhs),
            (Vector(lhs), Number(rhs)) => Vector(lhs / rhs),
            _ => panic!("Invalid Div"),
        },
        Lt(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Boolean(lhs < rhs),
            _ => panic!("Invalid Lt"),
        },
        Gt(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Boolean(lhs > rhs),
            _ => panic!("Invalid Gt"),
        },
        Min(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs.min(rhs)),
            _ => panic!("Invalid Min"),
        },
        Max(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs.max(rhs)),
            _ => panic!("Invalid Max"),
        },
        Mix(lhs, rhs, t) => match (
            evaluate_expr(this, lhs),
            evaluate_expr(this, rhs),
            evaluate_expr(this, t),
        ) {
            (Number(lhs), Number(rhs), Number(t)) => Number(lhs.mix(rhs, t)),
            (Vector(lhs), Vector(rhs), Number(t)) => Vector(lhs.mix(rhs, t)),
            _ => panic!("Invalid Mix"),
        },
        Neg(op) => match evaluate_expr(this, op) {
            Number(n) => Number(-n),
            Vector(v) => Vector(-v),
            _ => panic!("Invalid Neg"),
        },
        Abs(op) => match evaluate_expr(this, op) {
            Number(n) => Number(n.abs()),
            Vector(v) => Vector(v.abs()),
            _ => panic!("Invalid Abs"),
        },
        Sign(op) => match evaluate_expr(this, op) {
            Number(n) => Number(n.sign()),
            _ => panic!("Invalid Sign"),
        },
        Length(op) => match evaluate_expr(this, op) {
            Number(n) => Number(n),
            Vector(v) => Number(v.length()),
            _ => panic!("Invalid Length"),
        },
        Normalize(op) => match evaluate_expr(this, op) {
            Number(n) => Number(n.sign()),
            Vector(v) => Vector(v.normalize()),
            _ => panic!("Invalid Normalize"),
        },
        Dot(lhs, rhs) => match (evaluate_expr(this, lhs), evaluate_expr(this, rhs)) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            (Vector(lhs), Vector(rhs)) => Number(lhs.dot(rhs)),
            _ => panic!("Invalid Div"),
        },
    }
}
