mod into_block;

pub use into_block::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::ast::Stmt;

use super::TypeSpec;

/// List of statements
pub struct Block<T>(pub Vec<Stmt<T>>)
where
    T: TypeSpec;

impl<T> Debug for Block<T>
where
    T: TypeSpec,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}

impl<T> Default for Block<T>
where
    T: TypeSpec,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for Block<T>
where
    T: TypeSpec,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Hash for Block<T>
where
    T: TypeSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> IntoIterator for Block<T>
where
    T: TypeSpec,
{
    type Item = Stmt<T>;

    type IntoIter = std::vec::IntoIter<Stmt<T>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Block(ops) => ops,
        }
        .into_iter()
    }
}

impl<T> FromIterator<Stmt<T>> for Block<T>
where
    T: TypeSpec,
{
    fn from_iter<U: IntoIterator<Item = Stmt<T>>>(iter: U) -> Self {
        Block(iter.into_iter().collect())
    }
}

pub trait ComposeBlocks<T>: IntoIterator<Item = Stmt<T>>
where
    T: TypeSpec,
{
    fn compose<I: IntoIterator<Item = Stmt<T>>>(self, i: I) -> Stmt<T>;
}

pub fn into_block_stmt<T, I: IntoIterator<Item = Stmt<T>>>(t: I) -> Stmt<T>
where
    T: TypeSpec,
{
    Stmt::Block(crate::ir::ast::Block(t.into_iter().collect()))
}

impl<T, U> ComposeBlocks<U> for T
where
    U: TypeSpec,
    T: IntoIterator<Item = Stmt<U>>,
{
    fn compose<I: IntoIterator<Item = Stmt<U>>>(self, rhs: I) -> Stmt<U> {
        let Stmt::Block(lhs) = into_block_stmt(self) else {
            panic!("Compose LHS is not a Block");
        };
        let Stmt::Block(rhs) = into_block_stmt(rhs) else {
            panic!("Compose RHS is not a Block");
        };

        Stmt::Block(Block(
            Iterator::chain(lhs.into_iter(), rhs.into_iter()).collect(),
        ))
    }
}
