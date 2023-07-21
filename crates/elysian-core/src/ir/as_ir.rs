use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    ast::{Expr, Identifier},
    module::{FunctionDefinition, SpecializationData},
};

pub trait Domains {
    fn domains() -> Vec<Identifier> {
        Default::default()
    }
}

pub trait FilterSpecDyn {
    fn filter_spec_internal(&self) -> Vec<Identifier>;
}

impl<T> FilterSpecDyn for T
where
    T: Domains,
{
    fn filter_spec_internal(&self) -> Vec<Identifier> {
        T::domains()
    }
}

impl FilterSpecDyn for Box<dyn AsIR> {
    fn filter_spec_internal(&self) -> Vec<Identifier> {
        (**self).filter_spec_internal()
    }
}

pub trait AsIR: std::fmt::Debug + HashIR + CloneIR + FilterSpecDyn {
    fn functions(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        self.functions_impl(&spec.filter(self.filter_spec_internal()))
    }
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition>;

    fn expression(&self, spec: &SpecializationData, input: Expr) -> Expr {
        self.expression_impl(&spec.filter(self.filter_spec_internal()), input)
    }
    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr;
}

pub type DynAsIR = Box<dyn AsIR>;

impl AsIR for Box<dyn AsIR> {
    fn functions(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        (**self).functions(spec)
    }

    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition> {
        (**self).functions_impl(spec)
    }

    fn expression(&self, spec: &SpecializationData, input: Expr) -> Expr {
        (**self).expression(spec, input)
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr) -> Expr {
        (**self).expression_impl(spec, input)
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

impl HashIR for Box<dyn AsIR> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

pub trait CloneIR: 'static {
    fn clone_ir(&self) -> Box<dyn AsIR>;
}

impl<T> CloneIR for T
where
    T: 'static + Clone + AsIR,
{
    fn clone_ir(&self) -> Box<dyn AsIR> {
        Box::new(self.clone())
    }
}

impl CloneIR for Box<dyn AsIR> {
    fn clone_ir(&self) -> Box<dyn AsIR> {
        (**self).clone_ir()
    }
}
