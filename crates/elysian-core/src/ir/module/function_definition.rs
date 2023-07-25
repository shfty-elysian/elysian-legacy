use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use crate::ir::ast::{Block, Expr, Identifier, Stmt};

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

pub trait IntoRead {
    fn read(self) -> Expr;
}

impl<T> IntoRead for T
where
    T: IntoIterator<Item = PropertyIdentifier>,
{
    fn read(self) -> Expr {
        Expr::Read(self.into_iter().collect())
    }
}

pub trait IntoWrite {
    fn write(self, expr: Expr) -> Stmt;
}

impl<T> IntoWrite for T
where
    T: IntoIterator<Item = PropertyIdentifier>,
{
    fn write(self, expr: Expr) -> Stmt {
        Stmt::Write {
            path: self.into_iter().collect(),
            expr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub id: Identifier,
    pub public: bool,
    pub inputs: Vec<InputDefinition>,
    pub output: PropertyIdentifier,
    pub block: Block,
}

impl IntoIterator for FunctionDefinition {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl PartialEq for FunctionDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.public == other.public
            && self.inputs == other.inputs
            && self.output == other.output
    }
}

impl Hash for FunctionDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.public.hash(state);
        self.inputs.hash(state);
        self.output.hash(state);
        self.block.hash(state);
    }
}

impl FunctionDefinition {
    pub fn name(&self) -> &str {
        self.id.name()
    }

    pub fn name_unique(&self) -> String {
        self.id.name_unique()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputDefinition {
    pub id: PropertyIdentifier,
    pub mutable: bool,
}
