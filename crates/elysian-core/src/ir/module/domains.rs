use crate::ast::property_identifier::PropertyIdentifier;

pub trait Domains {
    fn domains() -> Vec<PropertyIdentifier> {
        Default::default()
    }
}

pub trait DomainsDyn {
    fn domains_dyn(&self) -> Vec<PropertyIdentifier>;
}

impl<T> DomainsDyn for T
where
    T: Domains,
{
    fn domains_dyn(&self) -> Vec<PropertyIdentifier> {
        T::domains()
    }
}
