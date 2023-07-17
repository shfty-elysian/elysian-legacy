use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{ast::Expr, module::FunctionDefinition};

pub trait AsIR<N, V>: std::fmt::Debug + HashIR + CloneIR<N, V> {
    fn functions(&self) -> Vec<FunctionDefinition<N, V>>;
    fn expression(&self, input: Expr<N, V>) -> Expr<N, V>;
}

impl<N, V> AsIR<N, V> for Box<dyn AsIR<N, V>>
where
    N: 'static,
    V: 'static,
{
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        (**self).functions()
    }

    fn expression(&self, input: Expr<N, V>) -> Expr<N, V> {
        (**self).expression(input)
    }
}

pub trait HashIR {
    fn hash_ir(&self) -> u64;
}

impl<T> HashIR for T
where
    T: Hash,
{
    fn hash_ir(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<N, V> HashIR for Box<dyn AsIR<N, V>> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

pub trait CloneIR<N, V>: 'static {
    fn clone_ir(&self) -> Box<dyn AsIR<N, V>>;
}

impl<T, N, V> CloneIR<N, V> for T
where
    T: 'static + Clone + AsIR<N, V>,
{
    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        Box::new(self.clone())
    }
}

impl<N, V> CloneIR<N, V> for Box<dyn AsIR<N, V>>
where
    N: 'static,
    V: 'static,
{
    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        (**self).clone_ir()
    }
}
