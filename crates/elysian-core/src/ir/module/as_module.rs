use std::{
    collections::{HashSet},
    fmt::Debug,
};

use crate::ir::{as_ir::HashIR, ast::Identifier};

use super::{FunctionDefinition, Module, SpecializationData, StructDefinition};

pub trait AsModule: 'static + Debug + HashIR {
    fn module(&self, spec: &SpecializationData) -> Module {
        let entry_point = self.entry_point();
        let mut functions = self.functions(spec, &entry_point);

        let mut set = HashSet::new();
        functions.retain(|x| set.insert(x.id.clone()));

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
    ) -> Vec<FunctionDefinition>;
    fn structs(&self) -> Vec<StructDefinition>;
}

pub type DynAsModule = Box<dyn AsModule>;

impl AsModule for DynAsModule {
    fn module(&self, spec: &SpecializationData) -> Module {
        (**self).module(spec)
    }

    fn entry_point(&self) -> Identifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &Identifier,
    ) -> Vec<FunctionDefinition> {
        (**self).functions(spec, entry_point)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }
}

impl HashIR for DynAsModule {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}
