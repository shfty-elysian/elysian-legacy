use std::{collections::BTreeMap, fmt::Debug};

use crate::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        ast::{Property, Value},
        module::StructDefinition,
    },
};

use super::{stmt::Stmt, Identifier};

/// Expression resulting in a value
pub enum Expr {
    Literal(Value),
    Vector2(BoxExpr, BoxExpr),
    Vector3(BoxExpr, BoxExpr, BoxExpr),
    Vector4(BoxExpr, BoxExpr, BoxExpr, BoxExpr),
    Struct(&'static StructDefinition, BTreeMap<Property, Expr>),
    Read(Vec<Property>),
    Call {
        function: Identifier,
        args: Vec<Expr>,
    },
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

impl IntoIterator for Expr {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Vector2(arg0, arg1) => f.debug_tuple("Vector2").field(arg0).field(arg1).finish(),
            Self::Vector3(arg0, arg1, arg2) => f
                .debug_tuple("Vector3")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Vector4(arg0, arg1, arg2, arg3) => f
                .debug_tuple("Vector4")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .field(arg3)
                .finish(),
            Self::Struct(arg0, arg1) => f.debug_tuple("Struct").field(arg0).field(arg1).finish(),
            Self::Read(arg0) => f.debug_tuple("Read").field(arg0).finish(),
            Self::Call { function, args } => f
                .debug_struct("Call")
                .field("function", function)
                .field("args", args)
                .finish(),
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
            Self::Vector2(arg0, arg1) => Self::Vector2(arg0.clone(), arg1.clone()),
            Self::Vector3(arg0, arg1, arg2) => {
                Self::Vector3(arg0.clone(), arg1.clone(), arg2.clone())
            }
            Self::Vector4(arg0, arg1, arg2, arg3) => {
                Self::Vector4(arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone())
            }
            Self::Struct(arg0, arg1) => Self::Struct(arg0.clone(), arg1.clone()),
            Self::Read(arg0) => Self::Read(arg0.clone()),
            Self::Call { function, args } => Self::Call {
                function: function.clone(),
                args: args.clone(),
            },
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

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Vector2(l0, l1), Self::Vector2(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Vector3(l0, l1, l2), Self::Vector3(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            (Self::Vector4(l0, l1, l2, l3), Self::Vector4(r0, r1, r2, r3)) => {
                l0 == r0 && l1 == r1 && l2 == r2 && l3 == r3
            }
            (Self::Struct(l0, l1), Self::Struct(r0, r1)) => l0 == r0 && l1 == r1,
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

impl From<ElysianExpr> for Expr {
    fn from(value: ElysianExpr) -> Self {
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

impl From<Box<ElysianExpr>> for Box<Expr> {
    fn from(value: Box<ElysianExpr>) -> Self {
        Box::new(Expr::from(*value))
    }
}

pub type BoxExpr = Box<Expr>;

impl core::ops::Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Add(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Sub(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mul(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Div(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Neg(Box::new(self))
    }
}

impl Expr {
    pub fn lt(self, rhs: Expr) -> Expr {
        Lt(Box::new(self), Box::new(rhs))
    }

    pub fn gt(self, rhs: Expr) -> Expr {
        Gt(Box::new(self), Box::new(rhs))
    }

    pub fn min(self, rhs: Expr) -> Expr {
        Min(Box::new(self), Box::new(rhs))
    }

    pub fn max(self, rhs: Expr) -> Expr {
        Max(Box::new(self), Box::new(rhs))
    }

    pub fn mix(self, rhs: Expr, t: Expr) -> Expr {
        Mix(Box::new(self), Box::new(rhs), Box::new(t))
    }

    pub fn dot(self, rhs: Expr) -> Expr {
        Dot(Box::new(self), Box::new(rhs))
    }

    pub fn abs(self) -> Expr {
        Abs(Box::new(self))
    }

    pub fn sign(self) -> Expr {
        Sign(Box::new(self))
    }

    pub fn length(self) -> Expr {
        Length(Box::new(self))
    }

    pub fn normalize(self) -> Expr {
        Normalize(Box::new(self))
    }

    pub fn output(self) -> Stmt {
        Stmt::Output(self)
    }

    pub fn vector2(x: Expr, y: Expr) -> Expr {
        Expr::Vector2(Box::new(x), Box::new(y))
    }

    pub fn vector3(x: Expr, y: Expr, z: Expr) -> Expr {
        Expr::Vector3(Box::new(x), Box::new(y), Box::new(z))
    }

    pub fn vector4(x: Expr, y: Expr, z: Expr, w: Expr) -> Expr {
        Expr::Vector4(Box::new(x), Box::new(y), Box::new(z), Box::new(w))
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
