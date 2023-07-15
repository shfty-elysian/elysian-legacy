mod into_block;

pub use into_block::*;

use std::fmt::Debug;

use crate::ir::ast::Stmt;

/// List of statements
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block<N, V>(pub Vec<Stmt<N, V>>);

impl<N, V> IntoIterator for Block<N, V> {
    type Item = Stmt<N, V>;

    type IntoIter = std::vec::IntoIter<Stmt<N, V>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Block(ops) => ops,
        }
        .into_iter()
    }
}

impl<N, V> FromIterator<Stmt<N, V>> for Block<N, V> {
    fn from_iter<T: IntoIterator<Item = Stmt<N, V>>>(iter: T) -> Self {
        Block(iter.into_iter().collect())
    }
}

pub trait ComposeBlocks<N, V>: IntoIterator<Item = Stmt<N, V>> {
    fn compose<I: IntoIterator<Item = Stmt<N, V>>>(self, i: I) -> Stmt<N, V>;
}

pub fn into_block_stmt<N, V, T: IntoIterator<Item = Stmt<N, V>>>(t: T) -> Stmt<N, V> {
    Stmt::Block(crate::ir::ast::Block(t.into_iter().collect()))
}

impl<N, V, T> ComposeBlocks<N, V> for T
where
    T: IntoIterator<Item = Stmt<N, V>>,
{
    fn compose<I: IntoIterator<Item = Stmt<N, V>>>(self, rhs: I) -> Stmt<N, V> {
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
