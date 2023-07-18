mod into_block;

pub use into_block::*;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::ast::Stmt;

use super::{TypeSpec, VectorSpace};

/// List of statements
pub struct Block<T, const N: usize>(pub Vec<Stmt<T, N>>)
where
    T: TypeSpec + VectorSpace<N>;

impl<T, const N: usize> Debug for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}

impl<T, const N: usize> Default for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T, const N: usize> Clone for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const N: usize> Hash for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, const N: usize> IntoIterator for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Item = Stmt<T, N>;

    type IntoIter = std::vec::IntoIter<Stmt<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Block(ops) => ops,
        }
        .into_iter()
    }
}

impl<T, const N: usize> FromIterator<Stmt<T, N>> for Block<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn from_iter<U: IntoIterator<Item = Stmt<T, N>>>(iter: U) -> Self {
        Block(iter.into_iter().collect())
    }
}

pub trait ComposeBlocks<T, const N: usize>: IntoIterator<Item = Stmt<T, N>>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn compose<I: IntoIterator<Item = Stmt<T, N>>>(self, i: I) -> Stmt<T, N>;
}

pub fn into_block_stmt<T, const N: usize, I: IntoIterator<Item = Stmt<T, N>>>(t: I) -> Stmt<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    Stmt::Block(crate::ir::ast::Block(t.into_iter().collect()))
}

impl<T, U, const N: usize> ComposeBlocks<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: IntoIterator<Item = Stmt<U, N>>,
{
    fn compose<I: IntoIterator<Item = Stmt<U, N>>>(self, rhs: I) -> Stmt<U, N> {
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
