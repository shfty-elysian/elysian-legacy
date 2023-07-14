use std::fmt::Debug;

use crate::ir::ast::{Expr, Property};

/// Statement consuming the result of an expression
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Stmt<N, V> {
    Block(crate::ir::ast::Block<N, V>),
    Write {
        path: Vec<Property>,
        expr: Expr<N, V>,
    },
    IfElse {
        cond: Expr<N, V>,
        then: Box<Stmt<N, V>>,
        otherwise: Box<Stmt<N, V>>,
    },
    Nop,
    Output(Expr<N, V>),
}

impl<N, V> Stmt<N, V> {
    pub fn if_else(self, cond: Expr<N, V>, otherwise: Stmt<N, V>) -> Self {
        Stmt::IfElse {
            cond,
            then: Box::new(self),
            otherwise: Box::new(otherwise),
        }
    }
}
