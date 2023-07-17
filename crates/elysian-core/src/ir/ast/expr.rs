use std::collections::BTreeMap;

use crate::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        ast::{IntoValue, Property, Value},
        module::StructDefinition,
    },
};

/// Expression resulting in a value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Expr<N, V> {
    Literal(Value<N, V>),
    Read(Vec<Property>),
    Call {
        function: Identifier,
        args: Vec<Expr<N, V>>,
    },
    Construct(&'static StructDefinition, BTreeMap<Property, Expr<N, V>>),
    Add(BoxExpr<N, V>, BoxExpr<N, V>),
    Sub(BoxExpr<N, V>, BoxExpr<N, V>),
    Mul(BoxExpr<N, V>, BoxExpr<N, V>),
    Div(BoxExpr<N, V>, BoxExpr<N, V>),
    Min(BoxExpr<N, V>, BoxExpr<N, V>),
    Max(BoxExpr<N, V>, BoxExpr<N, V>),
    Mix(BoxExpr<N, V>, BoxExpr<N, V>, BoxExpr<N, V>),
    Lt(BoxExpr<N, V>, BoxExpr<N, V>),
    Gt(BoxExpr<N, V>, BoxExpr<N, V>),
    Neg(BoxExpr<N, V>),
    Abs(BoxExpr<N, V>),
    Sign(BoxExpr<N, V>),
    Length(BoxExpr<N, V>),
    Normalize(BoxExpr<N, V>),
    Dot(BoxExpr<N, V>, BoxExpr<N, V>),
}

use Expr::*;

use super::{stmt::Stmt, Identifier};

impl<N, V> From<ElysianExpr<N, V>> for Expr<N, V> {
    fn from(value: ElysianExpr<N, V>) -> Self {
        match value {
            ElysianExpr::Literal(v) => Expr::Literal(v.into()),
            ElysianExpr::Read(p) => Expr::Read(vec![p.into()]),
            ElysianExpr::Add(lhs, rhs) => Expr::Add(lhs.into(), rhs.into()),
            ElysianExpr::Sub(lhs, rhs) => Expr::Sub(lhs.into(), rhs.into()),
            ElysianExpr::Mul(lhs, rhs) => Expr::Mul(lhs.into(), rhs.into()),
            ElysianExpr::Div(lhs, rhs) => Expr::Div(lhs.into(), rhs.into()),
            ElysianExpr::Min(lhs, rhs) => Expr::Min(lhs.into(), rhs.into()),
            ElysianExpr::Max(lhs, rhs) => Expr::Max(lhs.into(), rhs.into()),
            ElysianExpr::Mix(lhs, rhs, t) => Expr::Mix(lhs.into(), rhs.into(), t.into()),
            ElysianExpr::Lt(lhs, rhs) => Expr::Lt(lhs.into(), rhs.into()),
            ElysianExpr::Gt(lhs, rhs) => Expr::Gt(lhs.into(), rhs.into()),
            ElysianExpr::Neg(t) => Expr::Neg(t.into()),
            ElysianExpr::Abs(t) => Expr::Abs(t.into()),
            ElysianExpr::Sign(t) => Expr::Sign(t.into()),
            ElysianExpr::Length(t) => Expr::Length(t.into()),
            ElysianExpr::Normalize(t) => Expr::Normalize(t.into()),
            ElysianExpr::Dot(lhs, rhs) => Expr::Dot(lhs.into(), rhs.into()),
        }
    }
}

impl<N, V> From<Box<ElysianExpr<N, V>>> for Box<Expr<N, V>> {
    fn from(value: Box<ElysianExpr<N, V>>) -> Self {
        Box::new(Expr::from(*value))
    }
}

pub type BoxExpr<N, V> = Box<Expr<N, V>>;

impl<N, V> core::ops::Add for Expr<N, V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Add(Box::new(self), Box::new(rhs))
    }
}

impl<N, V> core::ops::Sub for Expr<N, V> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Sub(Box::new(self), Box::new(rhs))
    }
}

impl<N, V> core::ops::Mul for Expr<N, V> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mul(Box::new(self), Box::new(rhs))
    }
}

impl<N, V> core::ops::Div for Expr<N, V> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Div(Box::new(self), Box::new(rhs))
    }
}

impl<N, V> core::ops::Neg for Expr<N, V> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Neg(Box::new(self))
    }
}

impl<N, V> Expr<N, V> {
    pub fn lt(self, rhs: Expr<N, V>) -> Expr<N, V> {
        Lt(Box::new(self), Box::new(rhs))
    }

    pub fn gt(self, rhs: Expr<N, V>) -> Expr<N, V> {
        Gt(Box::new(self), Box::new(rhs))
    }

    pub fn min(self, rhs: Expr<N, V>) -> Expr<N, V> {
        Min(Box::new(self), Box::new(rhs))
    }

    pub fn max(self, rhs: Expr<N, V>) -> Expr<N, V> {
        Max(Box::new(self), Box::new(rhs))
    }

    pub fn mix(self, rhs: Expr<N, V>, t: Expr<N, V>) -> Expr<N, V> {
        Mix(Box::new(self), Box::new(rhs), Box::new(t))
    }

    pub fn dot(self, rhs: Expr<N, V>) -> Expr<N, V> {
        Dot(Box::new(self), Box::new(rhs))
    }

    pub fn abs(self) -> Expr<N, V> {
        Abs(Box::new(self))
    }

    pub fn sign(self) -> Expr<N, V> {
        Sign(Box::new(self))
    }

    pub fn length(self) -> Expr<N, V> {
        Length(Box::new(self))
    }

    pub fn normalize(self) -> Expr<N, V> {
        Normalize(Box::new(self))
    }

    pub fn output(self) -> Stmt<N, V> {
        Stmt::Output(self)
    }
}

pub trait IntoLiteral<N, V>: IntoValue<N, V> {
    fn literal(self) -> Expr<N, V>;
}

impl<T, N, V> IntoLiteral<N, V> for T
where
    T: IntoValue<N, V>,
{
    fn literal(self) -> Expr<N, V> {
        Expr::Literal(self.value())
    }
}
