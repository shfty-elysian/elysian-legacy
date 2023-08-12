use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use elysian_core::ast::identifier::Identifier;
use elysian_core::ast::property_identifier::PropertyIdentifier;
use elysian_core::ir::module::StructDefinition;
use elysian_proc_macros::{elysian_block, elysian_stmt};

use elysian_core::{
    ir::module::{
        AsIR, DomainsDyn, DynAsIR, FunctionDefinition, FunctionIdentifier, InputDefinition,
        IntoAsIR, SpecializationData, StructIdentifier, Type, CONTEXT,
    },
    property,
};

pub const FILTER_CONTEXT: Identifier = Identifier::new("filter_context", 11569410201650399545);
property!(
    FILTER_CONTEXT,
    FILTER_CONTEXT_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug)]
pub struct Filter {
    field: DynAsIR,
    props: Vec<PropertyIdentifier>,
}

impl Filter {
    pub fn new(
        field: impl IntoAsIR,
        props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>,
    ) -> Self {
        Filter {
            field: field.as_ir(),
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
    T: IntoAsIR,
{
    fn filter(self, props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Filter {
        Filter {
            field: self.as_ir(),
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}
