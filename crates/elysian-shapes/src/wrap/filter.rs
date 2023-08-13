use std::fmt::Debug;

use elysian_core::identifier::Identifier;
use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::ast::Expr;
use elysian_ir::module::Module;
use elysian_proc_macros::{elysian_block, elysian_stmt};

use elysian_ir::{
    module::{
        DomainsDyn, FunctionDefinition, FunctionIdentifier, InputDefinition, SpecializationData,
        StructIdentifier, Type, CONTEXT,
    },
    property,
};

use crate::wrap::Wrapper;
use crate::{shape::IntoShape, wrap::Wrap};

pub const FILTER_CONTEXT: Identifier = Identifier::new("filter_context", 11569410201650399545);
property!(
    FILTER_CONTEXT,
    FILTER_CONTEXT_PROP,
    Type::Struct(StructIdentifier(CONTEXT))
);

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    props: Vec<PropertyIdentifier>,
}

impl Filter {
    pub fn new(props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Self {
        Filter {
            props: props.into_iter().map(Into::into).collect(),
        }
    }
}

impl DomainsDyn for Filter {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        vec![]
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wrapper for Filter {
    fn module(&self, spec: &SpecializationData, field_call: Expr) -> elysian_ir::module::Module {
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

        Module::new(
            self,
            spec,
            FunctionDefinition {
                id: FunctionIdentifier::new_dynamic("filter".into()),
                public: false,
                inputs: vec![InputDefinition {
                    id: CONTEXT.into(),
                    mutable: true,
                }],
                output: CONTEXT.into(),
                block,
            },
        )
    }
}

pub trait IntoFilter {
    fn filter(self, props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Wrap;
}

impl<T> IntoFilter for T
where
    T: IntoShape,
{
    fn filter(self, props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> Wrap {
        Wrap::new(
            Filter {
                props: props.into_iter().map(Into::into).collect(),
            },
            self,
        )
    }
}
