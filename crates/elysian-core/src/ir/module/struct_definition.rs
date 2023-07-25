use std::borrow::Cow;

use crate::ir::ast::{Expr, Identifier};

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

    pub fn construct<I: IntoIterator<Item = (Identifier, Expr)>>(&self, props: I) -> Expr {
        Expr::Struct(self.id.clone(), props.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub id: Identifier,
    pub public: bool,
}
