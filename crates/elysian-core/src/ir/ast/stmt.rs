use std::fmt::Debug;

use crate::ir::ast::{Block, Expr, Property};

/// Statement consuming the result of an expression
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Stmt<N, V> {
    Block(Block<N, V>),
    Write {
        path: Vec<Property>,
        expr: Expr<N, V>,
    },
    If {
        cond: Expr<N, V>,
        then: Box<Stmt<N, V>>,
        otherwise: Option<Box<Stmt<N, V>>>,
    },
    Output(Expr<N, V>),
}

impl<N, V> Stmt<N, V> {
    pub fn if_else(self, cond: Expr<N, V>, otherwise: Option<Stmt<N, V>>) -> Self {
        Stmt::If {
            cond,
            then: Box::new(self),
            otherwise: otherwise.map(Box::new),
        }
    }
}
