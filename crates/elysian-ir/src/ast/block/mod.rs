mod into_block;

pub use into_block::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use crate::ast::Stmt;

/// List of statements
pub struct Block(pub Vec<Stmt>);

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}

impl Default for Block {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Deref for Block {
    type Target = Vec<Stmt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Block {
    type Item = Stmt;

    type IntoIter = std::vec::IntoIter<Stmt>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Block(ops) => ops,
        }
        .into_iter()
    }
}

impl FromIterator<Stmt> for Block {
    fn from_iter<U: IntoIterator<Item = Stmt>>(iter: U) -> Self {
        Block(iter.into_iter().collect())
    }
}

pub trait ComposeBlocks: IntoIterator<Item = Stmt> {
    fn compose<I: IntoIterator<Item = Stmt>>(self, i: I) -> Stmt;
}

pub fn into_block_stmt<I: IntoIterator<Item = Stmt>>(t: I) -> Stmt {
    Stmt::Block(crate::ast::Block(t.into_iter().collect()))
}

impl<T> ComposeBlocks for T
where
    T: IntoIterator<Item = Stmt>,
{
    fn compose<I: IntoIterator<Item = Stmt>>(self, rhs: I) -> Stmt {
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
