use std::hash::{Hash, Hasher};

use crate::ir::ast::{Block, Expr, Stmt};

use super::SpecializationData;

mod property_identifier;
pub use property_identifier::*;

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
    pub id: FunctionIdentifier,
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

mod function_identifier;
pub use function_identifier::*;
