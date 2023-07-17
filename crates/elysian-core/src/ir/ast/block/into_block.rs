use crate::ir::ast::{Block, Stmt};

pub trait IntoBlock<N, V>: IntoIterator<Item = Stmt<N, V>> {
    fn block(self) -> Block<N, V>;
}

impl<T, N, V> IntoBlock<N, V> for T
where
    T: IntoIterator<Item = Stmt<N, V>>,
{
    fn block(self) -> Block<N, V> {
        Block(self.into_iter().collect())
    }
}
