use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::ast::{Block, Expr, Property};

use super::{TypeSpec, VectorSpace};

/// Statement consuming the result of an expression
#[non_exhaustive]
pub enum Stmt<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    Block(Block<T, N>),
    Write {
        path: Vec<Property>,
        expr: Expr<T, N>,
    },
    If {
        cond: Expr<T, N>,
        then: Box<Stmt<T, N>>,
        otherwise: Option<Box<Stmt<T, N>>>,
    },
    Loop {
        stmt: Box<Stmt<T, N>>,
    },
    Break,
    Output(Expr<T, N>),
}

impl<T, const N: usize> Debug for Stmt<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Write { path, expr } => f
                .debug_struct("Write")
                .field("path", path)
                .field("expr", expr)
                .finish(),
            Self::If {
                cond,
                then,
                otherwise,
            } => f
                .debug_struct("If")
                .field("cond", cond)
                .field("then", then)
                .field("otherwise", otherwise)
                .finish(),
            Self::Loop { stmt } => f.debug_struct("Loop").field("stmt", stmt).finish(),
            Self::Break => f.debug_struct("Break").finish(),
            Self::Output(arg0) => f.debug_tuple("Output").field(arg0).finish(),
        }
    }
}

impl<T, const N: usize> Clone for Stmt<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Block(arg0) => Self::Block(arg0.clone()),
            Self::Write { path, expr } => Self::Write {
                path: path.clone(),
                expr: expr.clone(),
            },
            Self::If {
                cond,
                then,
                otherwise,
            } => Self::If {
                cond: cond.clone(),
                then: then.clone(),
                otherwise: otherwise.clone(),
            },
            Self::Loop { stmt } => Self::Loop { stmt: stmt.clone() },
            Self::Break => Self::Break,
            Self::Output(arg0) => Self::Output(arg0.clone()),
        }
    }
}

impl<T, const N: usize> Hash for Stmt<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<T, const N: usize> Stmt<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub fn if_else(self, cond: Expr<T, N>, otherwise: Option<Stmt<T, N>>) -> Self {
        Stmt::If {
            cond,
            then: Box::new(self),
            otherwise: otherwise.map(Box::new),
        }
    }
}
