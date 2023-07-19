use crate::ir::ast::{Expr, Identifier, Property, TypeSpec};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: Identifier,
    pub public: bool,
    pub fields: &'static [FieldDefinition],
}

impl StructDefinition {
    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }

    pub fn construct<T, I: IntoIterator<Item = (Property, Expr<T>)>>(
        &'static self,
        props: I,
    ) -> Expr<T>
    where
        T: TypeSpec,
    {
        Expr::Struct(self, props.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub prop: Property,
    pub public: bool,
}
