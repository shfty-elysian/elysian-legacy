use std::borrow::Cow;

use crate::ir::ast::{Expr, Identifier, Property};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: Identifier,
    pub public: bool,
    pub fields: Cow<'static, [FieldDefinition]>,
}

impl StructDefinition {
    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }

    pub fn construct<I: IntoIterator<Item = (Property, Expr)>>(&'static self, props: I) -> Expr {
        Expr::Struct(Cow::Borrowed(self), props.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub prop: Property,
    pub public: bool,
}
