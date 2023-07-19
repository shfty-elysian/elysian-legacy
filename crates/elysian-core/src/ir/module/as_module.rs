use std::fmt::Debug;

use crate::ir::{
    as_ir::HashIR,
    ast::{Identifier, TypeSpec},
};

use super::{FunctionDefinition, Module, SpecializationData, StructDefinition};

pub trait AsModule<T>: 'static + Debug + HashIR
where
    T: TypeSpec,
{
    fn module(&self, spec: &SpecializationData) -> Module<T> {
        let entry_point = self.entry_point();
        let mut functions = self.functions(spec, &entry_point);
        functions.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));
        functions.dedup_by(|lhs, rhs| lhs.id == rhs.id);

        Module {
            entry_point,
            struct_definitions: self.structs(),
            function_definitions: functions,
        }
    }

    fn entry_point(&self) -> Identifier;
    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition<T>>;
    fn structs(&self) -> Vec<StructDefinition>;
}

pub type DynAsModule<T> = Box<dyn AsModule<T>>;

impl<T> AsModule<T> for DynAsModule<T>
where
    T: TypeSpec,
{
    fn module(&self, spec: &SpecializationData) -> Module<T> {
        (**self).module(spec)
    }

    fn entry_point(&self) -> Identifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition<T>> {
        (**self).functions(spec, entry_point)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }
}

impl<T> HashIR for DynAsModule<T> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}
