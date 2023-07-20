use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    ast::{Expr, TypeSpec},
    module::{FunctionDefinition, SpecializationData},
};

pub trait FilterSpec {
    fn filter_spec(_spec: &SpecializationData) -> SpecializationData {
        Default::default()
    }
}

pub trait FilterSpecDyn {
    fn filter_spec_internal(&self, spec: &SpecializationData) -> SpecializationData;
}

impl<T> FilterSpecDyn for T
where
    T: FilterSpec,
{
    fn filter_spec_internal(&self, spec: &SpecializationData) -> SpecializationData {
        T::filter_spec(spec)
    }
}

impl<T> FilterSpecDyn for Box<dyn AsIR<T>> {
    fn filter_spec_internal(&self, spec: &SpecializationData) -> SpecializationData {
        (**self).filter_spec_internal(spec)
    }
}

pub trait AsIR<T>: std::fmt::Debug + HashIR + CloneIR<T> + FilterSpecDyn
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        self.functions_impl(&self.filter_spec_internal(spec))
    }
    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>>;

    fn expression(&self, spec: &SpecializationData, input: Expr<T>) -> Expr<T> {
        self.expression_impl(&self.filter_spec_internal(spec), input)
    }
    fn expression_impl(&self, spec: &SpecializationData, input: Expr<T>) -> Expr<T>;
}

pub type DynAsIR<T> = Box<dyn AsIR<T>>;

impl<T> AsIR<T> for Box<dyn AsIR<T>>
where
    T: TypeSpec,
{
    fn functions(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        (**self).functions(spec)
    }

    fn functions_impl(&self, spec: &SpecializationData) -> Vec<FunctionDefinition<T>> {
        (**self).functions_impl(spec)
    }

    fn expression(&self, spec: &SpecializationData, input: Expr<T>) -> Expr<T> {
        (**self).expression(spec, input)
    }

    fn expression_impl(&self, spec: &SpecializationData, input: Expr<T>) -> Expr<T> {
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
