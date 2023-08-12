use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_core::identifier::Identifier;
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::{Prepare, StructDefinition};
use elysian_proc_macros::{elysian_block, elysian_stmt};

use elysian_ir::{
    module::{
        AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, InputDefinition,
        SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};

use crate::shape::{DynShape, IntoShape};

pub const FILTER_CONTEXT: Identifier = Identifier::new("filter_context", 11569410201650399545);
property!(
    FILTER_CONTEXT,
    FILTER_CONTEXT_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Filter {
    field: DynShape,
    props: Vec<PropertyIdentifier>,
}

impl Filter {
    pub fn new(
        field: impl IntoShape,
        props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>,
    ) -> Self {
        Filter {
            field: field.shape(),
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}

impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.field.hash_ir());
        self.props.hash(state);
    }
}

impl DomainsDyn for Filter {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        self.field.domains_dyn()
    }
}

impl AsIR for Filter {
    fn entry_point(&self) -> FunctionIdentifier {
        FunctionIdentifier::new_dynamic("filter".into())
    }

    fn functions(
        &self,
        spec: &SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        let (_, field_call, field_functions) = self.field.call(spec, elysian_stmt! { CONTEXT });

        let mut block = elysian_block! {
            let FILTER_CONTEXT = #field_call;
        };

        for prop in &self.props {
            block.push(elysian_stmt! {
                CONTEXT.prop = FILTER_CONTEXT.prop
            });
        }

        block.push(elysian_stmt! {
            return CONTEXT
        });

        field_functions
            .into_iter()
            .chain(FunctionDefinition {
                id: entry_point.clone(),
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            })
            .collect()
    }

    fn structs(&self) -> Vec<StructDefinition> {
        self.field.structs()
    }
}

pub trait IntoFilter {
    fn filter(self, props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Filter;
}

impl<T> IntoFilter for T
where
    T: IntoShape,
{
    fn filter(self, props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Filter {
        Filter {
            field: self.shape(),
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}
