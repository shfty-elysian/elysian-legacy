use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    ast::{Expr, TypeSpec},
    module::FunctionDefinition,
};

pub trait AsIR<T>: std::fmt::Debug + HashIR + CloneIR<T>
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<FunctionDefinition<T>>;
    fn expression(&self, input: Expr<T>) -> Expr<T>;
}

pub type DynAsIR<T> = Box<dyn AsIR<T>>;

impl<T> AsIR<T> for Box<dyn AsIR<T>>
where
    T: TypeSpec,
{
    fn functions(&self) -> Vec<FunctionDefinition<T>> {
        (**self).functions()
    }

    fn expression(&self, input: Expr<T>) -> Expr<T> {
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

impl<T> HashIR for Box<dyn AsIR<T>> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

pub trait CloneIR<T>: 'static
where
    T: TypeSpec,
{
    fn clone_ir(&self) -> Box<dyn AsIR<T>>;
}

impl<T, U> CloneIR<U> for T
where
    U: TypeSpec,
    T: 'static + Clone + AsIR<U>,
{
    fn clone_ir(&self) -> Box<dyn AsIR<U>> {
        Box::new(self.clone())
    }
}

impl<T> CloneIR<T> for Box<dyn AsIR<T>>
where
    T: TypeSpec,
{
    fn clone_ir(&self) -> Box<dyn AsIR<T>> {
        (**self).clone_ir()
    }
}
