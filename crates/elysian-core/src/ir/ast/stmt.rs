use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::ast::{Block, Expr, Property};

use super::TypeSpec;

/// Statement consuming the result of an expression
#[non_exhaustive]
pub enum Stmt<T>
where
    T: TypeSpec,
{
    Block(Block<T>),
    Write {
        path: Vec<Property>,
        expr: Expr<T>,
        bind: bool,
    },
    If {
        cond: Expr<T>,
        then: Box<Stmt<T>>,
        otherwise: Option<Box<Stmt<T>>>,
    },
    Loop {
        stmt: Box<Stmt<T>>,
    },
    Break,
    Output(Expr<T>),
}

impl<T> IntoIterator for Stmt<T>
where
    T: TypeSpec,
{
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl<T> Debug for Stmt<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Write { path, expr, bind } => f
                .debug_struct("Write")
                .field("path", path)
                .field("expr", expr)
                .field("bind", bind)
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

impl<T> Clone for Stmt<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        match self {
            Self::Block(arg0) => Self::Block(arg0.clone()),
            Self::Write { path, expr, bind } => Self::Write {
                path: path.clone(),
                expr: expr.clone(),
                bind: bind.clone(),
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

impl<T> Hash for Stmt<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<T> Stmt<T>
where
    T: TypeSpec,
{
    pub fn if_else(self, cond: Expr<T>, otherwise: Option<Stmt<T>>) -> Self {
        Stmt::If {
            cond,
            then: Box::new(self),
            otherwise: otherwise.map(Box::new),
        }
    }
}
