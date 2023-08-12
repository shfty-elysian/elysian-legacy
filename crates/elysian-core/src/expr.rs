use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::identifier::Identifier;
use crate::property_identifier::PropertyIdentifier;

use super::value::Value;

pub type BoxExpr = Box<Expr>;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path(Vec<PropertyIdentifier>);

impl Path {
    pub fn push(mut self, seg: impl Into<PropertyIdentifier>) -> Self {
        self.0.push(seg.into());
        self
    }
    pub fn read(self) -> Expr {
        Expr::Read(self.0)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    Round(BoxExpr),
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
    And(BoxExpr, BoxExpr),
    Or(BoxExpr, BoxExpr),
    Min(BoxExpr, BoxExpr),
    Max(BoxExpr, BoxExpr),
    Dot(BoxExpr, BoxExpr),
    Mix(BoxExpr, BoxExpr, BoxExpr),
    Clamp(BoxExpr, BoxExpr, BoxExpr),
}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Expr {
    pub fn vector2(x: impl IntoExpr, y: impl IntoExpr) -> Expr {
        Expr::Vector2(x.box_expr(), y.box_expr())
    }

    pub fn vector3(x: impl IntoExpr, y: impl IntoExpr, z: impl IntoExpr) -> Expr {
        Expr::Vector3(x.box_expr(), y.box_expr(), z.box_expr())
    }

    pub fn vector4(x: impl IntoExpr, y: impl IntoExpr, z: impl IntoExpr, w: impl IntoExpr) -> Expr {
        Expr::Vector4(x.box_expr(), y.box_expr(), z.box_expr(), w.box_expr())
    }

    pub fn matrix2(x: impl IntoExpr, y: impl IntoExpr) -> Expr {
        Expr::Matrix2(x.box_expr(), y.box_expr())
    }

    pub fn matrix3(x: impl IntoExpr, y: impl IntoExpr, z: impl IntoExpr) -> Expr {
        Expr::Matrix3(x.box_expr(), y.box_expr(), z.box_expr())
    }

    pub fn matrix4(x: impl IntoExpr, y: impl IntoExpr, z: impl IntoExpr, w: impl IntoExpr) -> Expr {
        Expr::Matrix4(x.box_expr(), y.box_expr(), z.box_expr(), w.box_expr())
    }

    pub fn lt(self, rhs: impl IntoExpr) -> Expr {
        Expr::Lt(self.box_expr(), rhs.box_expr())
    }

    pub fn gt(self, rhs: impl IntoExpr) -> Expr {
        Expr::Gt(self.box_expr(), rhs.box_expr())
    }

    pub fn eq(self, rhs: impl IntoExpr) -> Expr {
        Expr::Eq(self.box_expr(), rhs.box_expr())
    }

    pub fn ne(self, rhs: impl IntoExpr) -> Expr {
        Expr::Ne(self.box_expr(), rhs.box_expr())
    }

    pub fn and(self, rhs: impl IntoExpr) -> Expr {
        Expr::And(self.box_expr(), rhs.box_expr())
    }

    pub fn or(self, rhs: impl IntoExpr) -> Expr {
        Expr::Or(self.box_expr(), rhs.box_expr())
    }

    pub fn abs(self) -> Expr {
        Expr::Abs(self.box_expr())
    }

    pub fn sign(self) -> Expr {
        Expr::Sign(self.box_expr())
    }

    pub fn round(self) -> Expr {
        Expr::Round(self.box_expr())
    }

    pub fn length(self) -> Expr {
        Expr::Length(self.box_expr())
    }

    pub fn normalize(self) -> Expr {
        Expr::Normalize(self.box_expr())
    }

    pub fn min(self, rhs: impl IntoExpr) -> Expr {
        Expr::Min(self.box_expr(), rhs.box_expr())
    }

    pub fn max(self, rhs: impl IntoExpr) -> Expr {
        Expr::Max(self.box_expr(), rhs.box_expr())
    }

    pub fn dot(self, rhs: impl IntoExpr) -> Expr {
        Expr::Dot(self.box_expr(), rhs.box_expr())
    }

    pub fn mix(self, rhs: impl IntoExpr, t: impl IntoExpr) -> Expr {
        Expr::Mix(self.box_expr(), rhs.box_expr(), t.box_expr())
    }

    pub fn clamp(self, min: impl IntoExpr, max: impl IntoExpr) -> Expr {
        Expr::Clamp(self.box_expr(), min.box_expr(), max.box_expr())
    }
}

impl<T> Add<T> for Expr
where
    T: IntoExpr,
{
    type Output = Expr;

    fn add(self, rhs: T) -> Self::Output {
        Expr::Add(self.box_expr(), rhs.box_expr())
    }
}

impl<T> Sub<T> for Expr
where
    T: IntoExpr,
{
    type Output = Expr;

    fn sub(self, rhs: T) -> Self::Output {
        Expr::Sub(self.box_expr(), rhs.box_expr())
    }
}

impl<T> Mul<T> for Expr
where
    T: IntoExpr,
{
    type Output = Expr;

    fn mul(self, rhs: T) -> Self::Output {
        Expr::Mul(self.box_expr(), rhs.box_expr())
    }
}

impl<T> Div<T> for Expr
where
    T: IntoExpr,
{
    type Output = Expr;

    fn div(self, rhs: T) -> Self::Output {
        Expr::Div(self.box_expr(), rhs.box_expr())
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expr::Neg(self.box_expr())
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

pub trait IntoExpr {
    fn expr(self) -> Expr;

    fn box_expr(self) -> BoxExpr
    where
        Self: Sized,
    {
        Box::new(self.expr())
    }
}

impl IntoExpr for Expr {
    fn expr(self) -> Expr {
        self
    }
}

impl<T> IntoExpr for T
where
    T: IntoLiteral,
{
    fn expr(self) -> Expr {
        self.literal()
    }
}

pub trait IntoPath {
    fn path(&self) -> Path;
}

impl IntoPath for Identifier {
    fn path(&self) -> Path {
        Path::default().push(PropertyIdentifier::from(self.clone()))
    }
}
