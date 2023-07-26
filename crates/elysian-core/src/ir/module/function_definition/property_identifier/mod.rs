use std::{
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::ir::ast::{Expr, Identifier, Stmt};

#[cfg(feature = "quote")]
mod to_tokens;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PropertyIdentifier(pub Identifier);

impl PropertyIdentifier {
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        PropertyIdentifier(Identifier::new(name, uuid))
    }

    pub fn bind(&self, expr: Expr) -> Stmt {
        Stmt::Bind {
            prop: self.clone(),
            expr,
        }
    }

    pub fn write(self, expr: Expr) -> Stmt {
        Stmt::Write {
            path: vec![self],
            expr,
        }
    }
}

impl Display for PropertyIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PropertyIdentifier({})", self.0)
    }
}

impl IntoIterator for PropertyIdentifier {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Deref for PropertyIdentifier {
    type Target = Identifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PropertyIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Identifier> for PropertyIdentifier {
    fn from(value: Identifier) -> Self {
        PropertyIdentifier(value)
    }
}

impl From<PropertyIdentifier> for Identifier {
    fn from(value: PropertyIdentifier) -> Self {
        value.0
    }
}
