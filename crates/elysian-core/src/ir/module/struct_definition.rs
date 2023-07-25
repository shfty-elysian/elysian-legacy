use std::{
    borrow::Cow,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::ir::ast::{Expr, Identifier};

use super::PropertyIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructIdentifier(pub Identifier);

impl StructIdentifier {
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        StructIdentifier(Identifier::new(name, uuid))
    }

    pub fn construct<I: IntoIterator<Item = (PropertyIdentifier, Expr)>>(&self, props: I) -> Expr {
        Expr::Struct(self.clone(), props.into_iter().collect())
    }
}

impl Display for StructIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StructIdentifier({})", self.0)
    }
}

impl IntoIterator for StructIdentifier {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Deref for StructIdentifier {
    type Target = Identifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StructIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Identifier> for StructIdentifier {
    fn from(value: Identifier) -> Self {
        StructIdentifier(value)
    }
}

impl From<StructIdentifier> for Identifier {
    fn from(value: StructIdentifier) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDefinition {
    pub id: StructIdentifier,
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    pub id: PropertyIdentifier,
    pub public: bool,
}
