use crate::ir::ast::{Block, Stmt, TypeSpec, VectorSpace};

pub trait IntoBlock<T, const N: usize>: IntoIterator<Item = Stmt<T, N>>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn block(self) -> Block<T, N>;
}

impl<T, U, const N: usize> IntoBlock<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: IntoIterator<Item = Stmt<U, N>>,
{
    fn block(self) -> Block<U, N> {
        Block(self.into_iter().collect())
    }
}
