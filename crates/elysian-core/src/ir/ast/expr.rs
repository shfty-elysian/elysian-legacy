use std::{collections::BTreeMap, fmt::Debug};

use crate::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        ast::{IntoValue, Property, Value},
        module::StructDefinition,
    },
};

/// Expression resulting in a value
#[non_exhaustive]
pub enum Expr<T, const N: usize>
where
    T: TypeSpec + VectorSpace<N>,
{
    Literal(Value<T, N>),
    Read(Vec<Property>),
    Call {
        function: Identifier,
        args: Vec<Expr<T, N>>,
    },
    Construct(&'static StructDefinition, BTreeMap<Property, Expr<T, N>>),
    Add(BoxExpr<T, N>, BoxExpr<T, N>),
    Sub(BoxExpr<T, N>, BoxExpr<T, N>),
    Mul(BoxExpr<T, N>, BoxExpr<T, N>),
    Div(BoxExpr<T, N>, BoxExpr<T, N>),
    Min(BoxExpr<T, N>, BoxExpr<T, N>),
    Max(BoxExpr<T, N>, BoxExpr<T, N>),
    Mix(BoxExpr<T, N>, BoxExpr<T, N>, BoxExpr<T, N>),
    Lt(BoxExpr<T, N>, BoxExpr<T, N>),
    Gt(BoxExpr<T, N>, BoxExpr<T, N>),
    Neg(BoxExpr<T, N>),
    Abs(BoxExpr<T, N>),
    Sign(BoxExpr<T, N>),
    Length(BoxExpr<T, N>),
    Normalize(BoxExpr<T, N>),
    Dot(BoxExpr<T, N>, BoxExpr<T, N>),
}

impl<T, const N: usize> Debug for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Read(arg0) => f.debug_tuple("Read").field(arg0).finish(),
            Self::Call { function, args } => f
                .debug_struct("Call")
                .field("function", function)
                .field("args", args)
                .finish(),
            Self::Construct(arg0, arg1) => {
                f.debug_tuple("Construct").field(arg0).field(arg1).finish()
            }
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

impl<T, const N: usize> Clone for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::Read(arg0) => Self::Read(arg0.clone()),
            Self::Call { function, args } => Self::Call {
                function: function.clone(),
                args: args.clone(),
            },
            Self::Construct(arg0, arg1) => Self::Construct(arg0.clone(), arg1.clone()),
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

impl<T, const N: usize> PartialEq for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Read(l0), Self::Read(r0)) => l0 == r0,
            (
                Self::Call {
                    function: l_function,
                    args: l_args,
                },
                Self::Call {
                    function: r_function,
                    args: r_args,
                },
            ) => l_function == r_function && l_args == r_args,
            (Self::Construct(l0, l1), Self::Construct(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Add(l0, l1), Self::Add(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Sub(l0, l1), Self::Sub(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Mul(l0, l1), Self::Mul(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Div(l0, l1), Self::Div(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Min(l0, l1), Self::Min(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Max(l0, l1), Self::Max(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Mix(l0, l1, l2), Self::Mix(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            (Self::Lt(l0, l1), Self::Lt(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Gt(l0, l1), Self::Gt(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Neg(l0), Self::Neg(r0)) => l0 == r0,
            (Self::Abs(l0), Self::Abs(r0)) => l0 == r0,
            (Self::Sign(l0), Self::Sign(r0)) => l0 == r0,
            (Self::Length(l0), Self::Length(r0)) => l0 == r0,
            (Self::Normalize(l0), Self::Normalize(r0)) => l0 == r0,
            (Self::Dot(l0, l1), Self::Dot(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

use Expr::*;

use super::{stmt::Stmt, Identifier, TypeSpec, VectorSpace};

impl<T, const N: usize> From<ElysianExpr<T>> for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn from(value: ElysianExpr<T>) -> Self {
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

impl<T, const N: usize> From<Box<ElysianExpr<T>>> for Box<Expr<T, N>>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn from(value: Box<ElysianExpr<T>>) -> Self {
        Box::new(Expr::from(*value))
    }
}

pub type BoxExpr<T, const N: usize> = Box<Expr<T, N>>;

impl<T, const N: usize> core::ops::Add for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Add(Box::new(self), Box::new(rhs))
    }
}

impl<T, const N: usize> core::ops::Sub for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Sub(Box::new(self), Box::new(rhs))
    }
}

impl<T, const N: usize> core::ops::Mul for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mul(Box::new(self), Box::new(rhs))
    }
}

impl<T, const N: usize> core::ops::Div for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Div(Box::new(self), Box::new(rhs))
    }
}

impl<T, const N: usize> core::ops::Neg for Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Neg(Box::new(self))
    }
}

impl<T, const N: usize> Expr<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub fn lt(self, rhs: Expr<T, N>) -> Expr<T, N> {
        Lt(Box::new(self), Box::new(rhs))
    }

    pub fn gt(self, rhs: Expr<T, N>) -> Expr<T, N> {
        Gt(Box::new(self), Box::new(rhs))
    }

    pub fn min(self, rhs: Expr<T, N>) -> Expr<T, N> {
        Min(Box::new(self), Box::new(rhs))
    }

    pub fn max(self, rhs: Expr<T, N>) -> Expr<T, N> {
        Max(Box::new(self), Box::new(rhs))
    }

    pub fn mix(self, rhs: Expr<T, N>, t: Expr<T, N>) -> Expr<T, N> {
        Mix(Box::new(self), Box::new(rhs), Box::new(t))
    }

    pub fn dot(self, rhs: Expr<T, N>) -> Expr<T, N> {
        Dot(Box::new(self), Box::new(rhs))
    }

    pub fn abs(self) -> Expr<T, N> {
        Abs(Box::new(self))
    }

    pub fn sign(self) -> Expr<T, N> {
        Sign(Box::new(self))
    }

    pub fn length(self) -> Expr<T, N> {
        Length(Box::new(self))
    }

    pub fn normalize(self) -> Expr<T, N> {
        Normalize(Box::new(self))
    }

    pub fn output(self) -> Stmt<T, N> {
        Stmt::Output(self)
    }
}

pub trait IntoLiteral<T, const N: usize>: IntoValue<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn literal(self) -> Expr<T, N>;
}

impl<T, U, const N: usize> IntoLiteral<U, N> for T
where
    U: TypeSpec + VectorSpace<N>,
    T: IntoValue<U, N>,
{
    fn literal(self) -> Expr<U, N> {
        Expr::Literal(self.value())
    }
}
