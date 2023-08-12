use elysian_core::property_identifier::PropertyIdentifier;
use elysian_ir::{
    ast::Expr,
    module::{AsIR, DomainsDyn, FunctionDefinition, FunctionIdentifier, HashIR, StructDefinition},
};

#[cfg_attr(feature = "serde", typetag::serialize(tag = "type"))]
pub trait Shape: AsIR + typetag::Serialize + typetag::Deserialize {}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", typetag::serialize)]
impl<T> Shape for T where T: AsIR + typetag::Serialize {}

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

impl AsIR for Box<dyn Shape> {
    fn entry_point(&self) -> FunctionIdentifier {
        (**self).entry_point()
    }

    fn functions(
        &self,
        spec: &elysian_ir::module::SpecializationData,
        entry_point: &FunctionIdentifier,
    ) -> Vec<FunctionDefinition> {
        (**self).functions(spec, entry_point)
    }

    fn arguments(&self, input: Expr) -> Vec<Expr> {
        (**self).arguments(input)
    }

    fn structs(&self) -> Vec<StructDefinition> {
        (**self).structs()
    }
}

pub trait IntoShape: 'static + Sized + Shape {
    fn shape(self) -> DynShape {
        Box::new(self)
    }
}

impl<T> IntoShape for T where T: 'static + Sized + Shape {}
