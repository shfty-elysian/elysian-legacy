use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::ir::module::PropertyIdentifier;

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
    Read(Vec<PropertyIdentifier>),
    Neg(BoxExpr),
    Abs(BoxExpr),
    Sign(BoxExpr),
    Length(BoxExpr),
    Normalize(BoxExpr),
    Add(BoxExpr, BoxExpr),
    Sub(BoxExpr, BoxExpr),
    Mul(BoxExpr, BoxExpr),
    Div(BoxExpr, BoxExpr),
    Eq(BoxExpr, BoxExpr),
    Ne(BoxExpr, BoxExpr),
    Lt(BoxExpr, BoxExpr),
    Gt(BoxExpr, BoxExpr),
    Min(BoxExpr, BoxExpr),
    Max(BoxExpr, BoxExpr),
    Dot(BoxExpr, BoxExpr),
    Mix(BoxExpr, BoxExpr, BoxExpr),
}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Expr {
    pub fn vector2(x: Expr, y: Expr) -> Expr {
        Expr::Vector2(Box::new(x), Box::new(y))
    }

    pub fn vector3(x: Expr, y: Expr, z: Expr) -> Expr {
        Expr::Vector3(Box::new(x), Box::new(y), Box::new(z))
    }

    pub fn vector4(x: Expr, y: Expr, z: Expr, w: Expr) -> Expr {
        Expr::Vector4(Box::new(x), Box::new(y), Box::new(z), Box::new(w))
    }

    pub fn matrix2(x: Expr, y: Expr) -> Expr {
        Expr::Matrix2(Box::new(x), Box::new(y))
    }

    pub fn matrix3(x: Expr, y: Expr, z: Expr) -> Expr {
        Expr::Matrix3(Box::new(x), Box::new(y), Box::new(z))
    }

    pub fn matrix4(x: Expr, y: Expr, z: Expr, w: Expr) -> Expr {
        Expr::Matrix4(Box::new(x), Box::new(y), Box::new(z), Box::new(w))
    }

    pub fn lt(self, rhs: Expr) -> Expr {
        Expr::Lt(Box::new(self), Box::new(rhs))
    }

    pub fn gt(self, rhs: Expr) -> Expr {
        Expr::Gt(Box::new(self), Box::new(rhs))
    }

    pub fn abs(self) -> Expr {
        Expr::Abs(Box::new(self))
    }

    pub fn sign(self) -> Expr {
        Expr::Sign(Box::new(self))
    }

    pub fn length(self) -> Expr {
        Expr::Length(Box::new(self))
    }

    pub fn normalize(self) -> Expr {
        Expr::Normalize(Box::new(self))
    }

    pub fn min(self, rhs: Expr) -> Expr {
        Expr::Min(Box::new(self), Box::new(rhs))
    }

    pub fn max(self, rhs: Expr) -> Expr {
        Expr::Max(Box::new(self), Box::new(rhs))
    }

    pub fn dot(self, rhs: Expr) -> Expr {
        Expr::Dot(Box::new(self), Box::new(rhs))
    }

    pub fn mix(self, rhs: Expr, t: Expr) -> Expr {
        Expr::Mix(Box::new(self), Box::new(rhs), Box::new(t))
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Self) -> Self::Output {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self))
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
