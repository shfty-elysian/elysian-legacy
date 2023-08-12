use std::{
    borrow::Cow,
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::ast::Expr;
use elysian_core::{ast::identifier::Identifier, uuid::Uuid};

use super::SpecializationData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionIdentifier(pub Identifier);

impl FunctionIdentifier {
    pub const fn new(name: &'static str, uuid: u128) -> Self {
        FunctionIdentifier(Identifier::new(name, uuid))
    }

    pub fn new_dynamic(name: Cow<'static, str>) -> Self {
        FunctionIdentifier(Identifier {
            name,
            uuid: Uuid::new_v4(),
        })
    }

    pub fn concat(&self, rhs: &Self) -> Self {
        FunctionIdentifier(self.0.concat(&rhs.0))
    }

    pub fn specialize(&self, spec: &SpecializationData) -> Self {
        FunctionIdentifier(spec.specialize_id(&self.0))
    }

    pub fn call<I: IntoIterator<Item = Expr>>(&self, args: I) -> Expr {
        Expr::Call {
            function: self.clone(),
            args: args.into_iter().collect(),
        }
    }
}

impl Display for FunctionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionIdentifier({})", self.0)
    }
}

impl IntoIterator for FunctionIdentifier {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Deref for FunctionIdentifier {
    type Target = Identifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FunctionIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Identifier> for FunctionIdentifier {
    fn from(value: Identifier) -> Self {
        FunctionIdentifier(value)
    }
}

impl From<FunctionIdentifier> for Identifier {
    fn from(value: FunctionIdentifier) -> Self {
        value.0
    }
}

pub trait IntoFunctionIdentifier {
    fn function(self) -> FunctionIdentifier;
}

impl IntoFunctionIdentifier for Identifier {
    fn function(self) -> FunctionIdentifier {
        self.into()
    }
}
