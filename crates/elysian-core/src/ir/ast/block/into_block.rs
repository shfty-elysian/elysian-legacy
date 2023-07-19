use crate::ir::ast::{Block, Stmt, TypeSpec};

pub trait IntoBlock<T>: IntoIterator<Item = Stmt<T>>
where
    T: TypeSpec,
{
    fn block(self) -> Block<T>;
}

impl<T, U> IntoBlock<U> for T
where
    U: TypeSpec,
    T: IntoIterator<Item = Stmt<U>>,
{
    fn block(self) -> Block<U> {
        Block(self.into_iter().collect())
    }
}
