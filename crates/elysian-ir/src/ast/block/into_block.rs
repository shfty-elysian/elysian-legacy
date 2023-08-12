use crate::ast::{Block, Stmt};

pub trait IntoBlock: IntoIterator<Item = Stmt> {
    fn block(self) -> Block;
}

impl<T> IntoBlock for T
where
    T: IntoIterator<Item = Stmt>,
{
    fn block(self) -> Block {
        Block(self.into_iter().collect())
    }
}
