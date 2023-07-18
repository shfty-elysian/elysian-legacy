use std::fmt::Debug;

use crate::ir::ast::TypeSpec;

use super::{
    attribute::Attribute,
    value::{IntoValue, Value},
};

pub type BoxExpr<T> = Box<Expr<T>>;
pub enum Expr<T>
where
    T: TypeSpec,
{
    Literal(Value<T>),
    Read(Attribute),
    Add(BoxExpr<T>, BoxExpr<T>),
    Sub(BoxExpr<T>, BoxExpr<T>),
    Mul(BoxExpr<T>, BoxExpr<T>),
    Div(BoxExpr<T>, BoxExpr<T>),
    Min(BoxExpr<T>, BoxExpr<T>),
    Max(BoxExpr<T>, BoxExpr<T>),
    Mix(BoxExpr<T>, BoxExpr<T>, BoxExpr<T>),
    Lt(BoxExpr<T>, BoxExpr<T>),
    Gt(BoxExpr<T>, BoxExpr<T>),
    Neg(BoxExpr<T>),
    Abs(BoxExpr<T>),
    Sign(BoxExpr<T>),
    Length(BoxExpr<T>),
    Normalize(BoxExpr<T>),
    Dot(BoxExpr<T>, BoxExpr<T>),
}

impl<T> Debug for Expr<T>
where
    T: TypeSpec,
{
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

impl<T> Clone for Expr<T>
where
    T: TypeSpec,
{
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

impl<T> std::hash::Hash for Expr<T>
where
    T: TypeSpec,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait IntoLiteral<T>: IntoValue<T>
where
    T: TypeSpec,
{
    fn literal(self) -> Expr<T>;
}

impl<T, U> IntoLiteral<U> for T
where
    U: TypeSpec,
    T: IntoValue<U>,
{
    fn literal(self) -> Expr<U> {
        Expr::Literal(self.value())
    }
}
