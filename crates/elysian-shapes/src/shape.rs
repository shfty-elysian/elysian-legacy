use std::fmt::Debug;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::{AsModule, DomainsDyn, ErasedHash};

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Shape: Debug + AsModule + ErasedHash + DomainsDyn {}

pub type DynShape = Box<dyn Shape>;

impl ErasedHash for Box<dyn Shape> {
    fn erased_hash(&self) -> u64 {
        (**self).erased_hash()
    }
}

impl DomainsDyn for Box<dyn Shape> {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        (**self).domains_dyn()
    }
}

impl AsModule for Box<dyn Shape> {
    fn module(&self, spec: &elysian_ir::module::SpecializationData) -> elysian_ir::module::Module {
        (**self).module(spec)
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Shape for Box<dyn Shape> {}

pub trait IntoShape: 'static + Sized + Shape {
    fn shape(self) -> DynShape {
        Box::new(self)
    }
}

impl<T> IntoShape for T where T: 'static + Sized + Shape {}
