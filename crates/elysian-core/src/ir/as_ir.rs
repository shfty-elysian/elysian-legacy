use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    ast::{Expr, TypeSpec, VectorSpace},
    module::FunctionDefinition,
};

pub trait AsIR<T, const N: usize>: std::fmt::Debug + HashIR + CloneIR<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>>;
    fn expression(&self, input: Expr<T, N>) -> Expr<T, N>;
}

impl<T, const N: usize> AsIR<T, N> for Box<dyn AsIR<T, N>>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn functions(&self) -> Vec<FunctionDefinition<T, N>> {
        (**self).functions()
    }

    fn expression(&self, input: Expr<T, N>) -> Expr<T, N> {
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

impl<T, const N: usize> HashIR for Box<dyn AsIR<T, N>> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

pub trait CloneIR<T, const N: usize>: 'static
where
    T: TypeSpec,
{
    fn clone_ir(&self) -> Box<dyn AsIR<T, N>>;
}

impl<T, U, const N: usize> CloneIR<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: 'static + Clone + AsIR<U, N>,
{
    fn clone_ir(&self) -> Box<dyn AsIR<U, N>> {
        Box::new(self.clone())
    }
}

impl<T, const N: usize> CloneIR<T, N> for Box<dyn AsIR<T, N>>
where
    T: TypeSpec,
{
    fn clone_ir(&self) -> Box<dyn AsIR<T, N>> {
        (**self).clone_ir()
    }
}
