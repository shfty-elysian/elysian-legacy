use std::fmt::Debug;

use crate::ir::{
    as_ir::HashIR,
    ast::{Identifier, VectorSpace},
};

use super::{FunctionDefinition, Module, StructDefinition};

pub trait AsModule<T, const N: usize>: 'static + Debug + HashIR
where
    T: VectorSpace<N>,
{
    fn module(&self) -> Module<T, N> {
        let entry_point = self.entry_point();
        let mut functions = self.functions(entry_point.clone());
        functions.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id));
        functions.dedup_by(|lhs, rhs| lhs.id == rhs.id);

        Module {
            entry_point,
            struct_definitions: self.structs(),
            function_definitions: functions,
        }
    }

    fn entry_point(&self) -> Identifier;
    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>>;
    fn structs(&self) -> Vec<StructDefinition>;
}

pub type DynAsModule<T, const N: usize> = Box<dyn AsModule<T, N>>;

impl<T, const N: usize> AsModule<T, N> for DynAsModule<T, N>
where
    T: VectorSpace<N>,
{
    fn module(&self) -> Module<T, N> {
        (**self).module()
    }

    fn entry_point(&self) -> Identifier {
        (**self).entry_point()
    }

    fn functions(&self, entry_point: Identifier) -> Vec<FunctionDefinition<T, N>> {
        (**self).functions(entry_point)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }
}

impl<T, const N: usize> HashIR for DynAsModule<T, N> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}
