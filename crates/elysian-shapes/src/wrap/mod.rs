pub mod cross_section;
pub mod filter;
pub mod rotate;
pub mod elongate_basis;
pub mod mirror;
pub mod scale;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use elysian_ir::{
    ast::Expr,
    module::{AsModule, DomainsDyn, ErasedHash, Module, SpecializationData, CONTEXT},
};
use elysian_proc_macros::elysian_stmt;

use crate::shape::{DynShape, IntoShape, Shape};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wrap {
    wrapper: Box<dyn Wrapper>,
    shape: DynShape,
}

impl Wrap {
    pub fn new(wrapper: impl Wrapper, shape: impl IntoShape) -> Self {
        Wrap {
            wrapper: Box::new(wrapper),
            shape: shape.shape(),
        }
    }
}

#[typetag::serde]
pub trait Wrapper: 'static + Debug + ErasedHash + DomainsDyn {
    fn module(&self, spec: &SpecializationData, field_call: Expr) -> Module;
}

impl Hash for Wrap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.wrapper.erased_hash());
        state.write_u64(self.shape.erased_hash());
    }
}

impl DomainsDyn for Wrap {
    fn domains_dyn(&self) -> Vec<elysian_core::property_identifier::PropertyIdentifier> {
        self.shape
            .domains_dyn()
            .into_iter()
            .chain(self.wrapper.domains_dyn())
            .collect()
    }
}

impl AsModule for Wrap {
    fn module(&self, spec: &SpecializationData) -> elysian_ir::module::Module {
        let shape_module = self.shape.module(spec);
        let field_call = shape_module.call(elysian_stmt! { CONTEXT });

        let wrapper_module = self.wrapper.module(spec, field_call);

        shape_module.concat(wrapper_module)
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Wrap {}
