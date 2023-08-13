use std::fmt::Debug;

use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::module::{AsModule, DomainsDyn, HashIR};

#[cfg_attr(feature = "serde", typetag::serialize(tag = "type"))]
pub trait Shape:
    Debug + AsModule + HashIR + DomainsDyn + typetag::Serialize + typetag::Deserialize
{
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", typetag::serialize)]
impl<T> Shape for T where T: Debug + AsModule + HashIR + DomainsDyn + typetag::Serialize {}

#[cfg(not(feature = "serde"))]
impl<T> Shape for T where T: AsIR {}

pub type DynShape = Box<dyn Shape>;

impl HashIR for Box<dyn Shape> {
    fn hash_ir(&self) -> u64 {
        (**self).hash_ir()
    }
}

impl DomainsDyn for Box<dyn Shape> {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        (**self).domains_dyn()
    }
}

impl AsModule for Box<dyn Shape> {
    fn module_impl(
        &self,
        spec: &elysian_ir::module::SpecializationData,
    ) -> elysian_ir::module::Module {
        (**self).module_impl(spec)
    }

    fn module(&self, spec: &elysian_ir::module::SpecializationData) -> elysian_ir::module::Module {
        (**self).module(spec)
    }
}

pub trait IntoShape: 'static + Sized + Shape {
    fn shape(self) -> DynShape {
        Box::new(self)
    }
}

impl<T> IntoShape for T where T: 'static + Sized + Shape {}
