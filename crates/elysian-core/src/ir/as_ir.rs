use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{ast::Expr, module::FunctionDefinition};

pub trait AsIR<N, V>: std::fmt::Debug {
    fn functions(&self) -> Vec<FunctionDefinition<N, V>>;
    fn expressions(&self, input: Expr<N, V>) -> Vec<Expr<N, V>>;
    fn hash_ir(&self) -> u64;
    fn clone_ir(&self) -> Box<dyn AsIR<N, V>>;
}

pub fn hash_ir(ir: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    ir.hash(&mut hasher);
    hasher.finish()
}

pub fn clone_ir<N, V>(ir: &(impl AsIR<N, V> + Clone + 'static)) -> Box<dyn AsIR<N, V>> {
    Box::new(ir.clone())
}

impl<T, N, V> AsIR<N, V> for &T
where
    T: AsIR<N, V>,
{
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        (*self).functions()
    }

    fn expressions(&self, input: Expr<N, V>) -> Vec<Expr<N, V>> {
        (*self).expressions(input)
    }

    fn hash_ir(&self) -> u64 {
        (*self).hash_ir()
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        (*self).clone_ir()
    }
}

impl<N, V> AsIR<N, V> for Box<dyn AsIR<N, V>> {
    fn functions(&self) -> Vec<FunctionDefinition<N, V>> {
        (**self).functions()
    }

    fn expressions(&self, input: Expr<N, V>) -> Vec<Expr<N, V>> {
        (**self).expressions(input)
    }

    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }

    fn clone_ir(&self) -> Box<dyn AsIR<N, V>> {
        (**self).clone_ir()
    }
}
