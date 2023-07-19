pub mod attribute;
pub mod central_diff_gradient;
pub mod combine;
pub mod expr;
pub mod field;
pub mod modify;
pub mod value;
pub mod raymarch {
    use std::{
        fmt::Debug,
        hash::{Hash, Hasher},
    };

    use crate::ir::{
        ast::{GlamF32, Identifier},
        module::{AsModule, SpecializationData},
    };

    pub struct Raymarch {
        pub field: Box<dyn AsModule<GlamF32>>,
    }

    impl Debug for Raymarch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Raymarch")
                .field("field", &self.field)
                .finish()
        }
    }

    impl Hash for Raymarch {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u64(self.field.hash_ir())
        }
    }

    impl AsModule<GlamF32> for Raymarch {
        fn entry_point(&self) -> Identifier {
            Identifier::new_dynamic("raymarch")
        }

        fn functions(
            &self,
            spec: &SpecializationData,
            _: &Identifier,
        ) -> Vec<crate::ir::module::FunctionDefinition<GlamF32>> {
            let field_entry_point = self.field.entry_point();
            self.field.functions(spec, &field_entry_point)
        }

        fn structs(&self) -> Vec<crate::ir::module::StructDefinition> {
            self.field.structs()
        }
    }
}

use crate::ir::{as_ir::DynAsIR, module::DynAsModule};

use self::combine::Combine;

pub trait IntoCombine<T> {
    fn combine<U>(self, combinator: U) -> Combine<T>
    where
        U: IntoIterator<Item = DynAsIR<T>>;
}

impl<T, U> IntoCombine<U> for T
where
    T: IntoIterator<Item = DynAsModule<U>>,
{
    fn combine<V>(self, combinator: V) -> Combine<U>
    where
        V: IntoIterator<Item = DynAsIR<U>>,
    {
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}
