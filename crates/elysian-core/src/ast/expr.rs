use std::fmt::Debug;

use super::{attribute::Attribute, value::Value};

pub type BoxExpr = Box<Expr>;
pub enum Expr {
    Literal(Value),
    Read(Attribute),
    Add(BoxExpr, BoxExpr),
    Sub(BoxExpr, BoxExpr),
    Mul(BoxExpr, BoxExpr),
    Div(BoxExpr, BoxExpr),
    Min(BoxExpr, BoxExpr),
    Max(BoxExpr, BoxExpr),
    Mix(BoxExpr, BoxExpr, BoxExpr),
    Lt(BoxExpr, BoxExpr),
    Gt(BoxExpr, BoxExpr),
    Neg(BoxExpr),
    Abs(BoxExpr),
    Sign(BoxExpr),
    Length(BoxExpr),
    Normalize(BoxExpr),
    Dot(BoxExpr, BoxExpr),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Read(arg0) => f.debug_tuple("Read").field(arg0).finish(),
            Self::Add(arg0, arg1) => f.debug_tuple("Add").field(arg0).field(arg1).finish(),
            Self::Sub(arg0, arg1) => f.debug_tuple("Sub").field(arg0).field(arg1).finish(),
            Self::Mul(arg0, arg1) => f.debug_tuple("Mul").field(arg0).field(arg1).finish(),
            Self::Div(arg0, arg1) => f.debug_tuple("Div").field(arg0).field(arg1).finish(),
            Self::Min(arg0, arg1) => f.debug_tuple("Min").field(arg0).field(arg1).finish(),
            Self::Max(arg0, arg1) => f.debug_tuple("Max").field(arg0).field(arg1).finish(),
            Self::Mix(arg0, arg1, arg2) => f
                .debug_tuple("Mix")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Lt(arg0, arg1) => f.debug_tuple("Lt").field(arg0).field(arg1).finish(),
            Self::Gt(arg0, arg1) => f.debug_tuple("Gt").field(arg0).field(arg1).finish(),
            Self::Neg(arg0) => f.debug_tuple("Neg").field(arg0).finish(),
            Self::Abs(arg0) => f.debug_tuple("Abs").field(arg0).finish(),
            Self::Sign(arg0) => f.debug_tuple("Sign").field(arg0).finish(),
            Self::Length(arg0) => f.debug_tuple("Length").field(arg0).finish(),
            Self::Normalize(arg0) => f.debug_tuple("Normalize").field(arg0).finish(),
            Self::Dot(arg0, arg1) => f.debug_tuple("Dot").field(arg0).field(arg1).finish(),
        }
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::Read(arg0) => Self::Read(arg0.clone()),
            Self::Add(arg0, arg1) => Self::Add(arg0.clone(), arg1.clone()),
            Self::Sub(arg0, arg1) => Self::Sub(arg0.clone(), arg1.clone()),
            Self::Mul(arg0, arg1) => Self::Mul(arg0.clone(), arg1.clone()),
            Self::Div(arg0, arg1) => Self::Div(arg0.clone(), arg1.clone()),
            Self::Min(arg0, arg1) => Self::Min(arg0.clone(), arg1.clone()),
            Self::Max(arg0, arg1) => Self::Max(arg0.clone(), arg1.clone()),
            Self::Mix(arg0, arg1, arg2) => Self::Mix(arg0.clone(), arg1.clone(), arg2.clone()),
            Self::Lt(arg0, arg1) => Self::Lt(arg0.clone(), arg1.clone()),
            Self::Gt(arg0, arg1) => Self::Gt(arg0.clone(), arg1.clone()),
            Self::Neg(arg0) => Self::Neg(arg0.clone()),
            Self::Abs(arg0) => Self::Abs(arg0.clone()),
            Self::Sign(arg0) => Self::Sign(arg0.clone()),
            Self::Length(arg0) => Self::Length(arg0.clone()),
            Self::Normalize(arg0) => Self::Normalize(arg0.clone()),
            Self::Dot(arg0, arg1) => Self::Dot(arg0.clone(), arg1.clone()),
        }
    }
}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait IntoLiteral: Into<Value> {
    fn literal(self) -> Expr;
}

impl<T> IntoLiteral for T
where
    T: Into<Value>,
{
    fn literal(self) -> Expr {
        Expr::Literal(self.into())
    }
}
