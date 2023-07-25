use std::fmt::Debug;

use crate::ir::ast::Identifier;

use super::value::Value;

pub type BoxExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Vector2(BoxExpr, BoxExpr),
    Vector3(BoxExpr, BoxExpr, BoxExpr),
    Vector4(BoxExpr, BoxExpr, BoxExpr, BoxExpr),
    Matrix2(BoxExpr, BoxExpr),
    Matrix3(BoxExpr, BoxExpr, BoxExpr),
    Matrix4(BoxExpr, BoxExpr, BoxExpr, BoxExpr),
    Read(Vec<Identifier>),
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
