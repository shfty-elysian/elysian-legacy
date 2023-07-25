use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::ast::{Block, Expr};

use super::Identifier;

/// Statement consuming the result of an expression
pub enum Stmt {
    Block(Block),
    Bind {
        prop: Identifier,
        expr: Expr,
    },
    Write {
        path: Vec<Identifier>,
        expr: Expr,
    },
    If {
        cond: Expr,
        then: Box<Stmt>,
        otherwise: Option<Box<Stmt>>,
    },
    Loop {
        stmt: Box<Stmt>,
    },
    Break,
    Output(Expr),
}

impl IntoIterator for Stmt {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Bind { prop, expr } => f
                .debug_struct("Bind")
                .field("prop", prop)
                .field("expr", expr)
                .finish(),
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
            Self::Break => write!(f, "Break"),
            Self::Output(arg0) => f.debug_tuple("Output").field(arg0).finish(),
        }
    }
}

impl Clone for Stmt {
    fn clone(&self) -> Self {
        match self {
            Self::Block(arg0) => Self::Block(arg0.clone()),
            Self::Bind { prop, expr } => Self::Bind {
                prop: prop.clone(),
                expr: expr.clone(),
            },
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

impl Hash for Stmt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Stmt {
    pub fn if_else(self, cond: Expr, otherwise: Option<Stmt>) -> Self {
        Stmt::If {
            cond,
            then: Box::new(self),
            otherwise: otherwise.map(Box::new),
        }
    }
}
