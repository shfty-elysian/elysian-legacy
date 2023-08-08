use std::fmt::Debug;

use crate::{
    ast::expr::Expr as ElysianExpr,
    ir::{
        ast::{
            Value, W_AXIS_4, X_AXIS_2, X_AXIS_3, X_AXIS_4, Y_AXIS_2, Y_AXIS_3, Y_AXIS_4, Z_AXIS_3,
            Z_AXIS_4,
        },
        module::{
            FunctionDefinition, FunctionIdentifier, NumericType, PropertyIdentifier,
            StructIdentifier, Type, CONTEXT,
        },
    },
};

use super::{stmt::Stmt, MATRIX2, MATRIX3, MATRIX4, VECTOR2, VECTOR3, VECTOR4, W, X, Y, Z};

/// Expression resulting in a value
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Struct(StructIdentifier, IndexMap<PropertyIdentifier, Expr>),
    Read(Vec<PropertyIdentifier>),
    Call {
        function: FunctionIdentifier,
        args: Vec<Expr>,
    },
    Neg(BoxExpr),
    Abs(BoxExpr),
    Sign(BoxExpr),
    Round(BoxExpr),
    Sin(BoxExpr),
    Cos(BoxExpr),
    Tan(BoxExpr),
    Asin(BoxExpr),
    Acos(BoxExpr),
    Atan(BoxExpr),
    Length(BoxExpr),
    Normalize(BoxExpr),
    Add(BoxExpr, BoxExpr),
    Sub(BoxExpr, BoxExpr),
    Mul(BoxExpr, BoxExpr),
    Div(BoxExpr, BoxExpr),
    Mod(BoxExpr, BoxExpr),
    Eq(BoxExpr, BoxExpr),
    Ne(BoxExpr, BoxExpr),
    Lt(BoxExpr, BoxExpr),
    Gt(BoxExpr, BoxExpr),
    And(BoxExpr, BoxExpr),
    Or(BoxExpr, BoxExpr),
    Min(BoxExpr, BoxExpr),
    Max(BoxExpr, BoxExpr),
    Dot(BoxExpr, BoxExpr),
    Atan2(BoxExpr, BoxExpr),
    Mix(BoxExpr, BoxExpr, BoxExpr),
    Clamp(BoxExpr, BoxExpr, BoxExpr),
}

impl IntoIterator for Expr {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use elysian_proc_macros::elysian_expr;
use indexmap::IndexMap;
use Expr::*;

impl From<ElysianExpr> for Expr {
    fn from(value: ElysianExpr) -> Self {
        match value {
            ElysianExpr::Literal(v) => Expr::Literal(v.into()),
            ElysianExpr::Vector2(x, y) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                elysian_expr! {
                    VECTOR2 {
                        X: #x,
                        Y: #y
                    }
                }
            }
            ElysianExpr::Vector3(x, y, z) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                let z = Expr::from((*z).clone());
                elysian_expr! {
                    VECTOR3 {
                        X: #x,
                        Y: #y,
                        Z: #z
                    }
                }
            }
            ElysianExpr::Vector4(x, y, z, w) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                let z = Expr::from((*z).clone());
                let w = Expr::from((*w).clone());
                elysian_expr! {
                    VECTOR4 {
                        X: #x,
                        Y: #y,
                        Z: #z,
                        W: #w
                    }
                }
            }
            ElysianExpr::Matrix2(x, y) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                elysian_expr! {
                    MATRIX2 {
                        X_AXIS_2: #x,
                        Y_AXIS_2: #y,
                    }
                }
            }
            ElysianExpr::Matrix3(x, y, z) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                let z = Expr::from((*z).clone());
                elysian_expr! {
                    MATRIX3 {
                        X_AXIS_3: #x,
                        Y_AXIS_3: #y,
                        Z_AXIS_3: #z,
                    }
                }
            }
            ElysianExpr::Matrix4(x, y, z, w) => {
                let x = Expr::from((*x).clone());
                let y = Expr::from((*y).clone());
                let z = Expr::from((*z).clone());
                let w = Expr::from((*w).clone());
                elysian_expr! {
                    MATRIX4 {
                        X_AXIS_4: #x,
                        Y_AXIS_4: #y,
                        Z_AXIS_4: #z,
                        W_AXIS_4: #w,
                    }
                }
            }
            ElysianExpr::Read(p) => {
                Expr::Read([PropertyIdentifier(CONTEXT)].into_iter().chain(p).collect())
            }
            ElysianExpr::Neg(t) => Expr::Neg(t.into()),
            ElysianExpr::Abs(t) => Expr::Abs(t.into()),
            ElysianExpr::Sign(t) => Expr::Sign(t.into()),
            ElysianExpr::Round(t) => Expr::Round(t.into()),
            ElysianExpr::Length(t) => Expr::Length(t.into()),
            ElysianExpr::Normalize(t) => Expr::Normalize(t.into()),
            ElysianExpr::Add(lhs, rhs) => Expr::Add(lhs.into(), rhs.into()),
            ElysianExpr::Sub(lhs, rhs) => Expr::Sub(lhs.into(), rhs.into()),
            ElysianExpr::Mul(lhs, rhs) => Expr::Mul(lhs.into(), rhs.into()),
            ElysianExpr::Div(lhs, rhs) => Expr::Div(lhs.into(), rhs.into()),
            ElysianExpr::Min(lhs, rhs) => Expr::Min(lhs.into(), rhs.into()),
            ElysianExpr::Max(lhs, rhs) => Expr::Max(lhs.into(), rhs.into()),
            ElysianExpr::Eq(lhs, rhs) => Expr::Eq(lhs.into(), rhs.into()),
            ElysianExpr::Ne(lhs, rhs) => Expr::Ne(lhs.into(), rhs.into()),
            ElysianExpr::Lt(lhs, rhs) => Expr::Lt(lhs.into(), rhs.into()),
            ElysianExpr::Gt(lhs, rhs) => Expr::Gt(lhs.into(), rhs.into()),
            ElysianExpr::And(lhs, rhs) => Expr::And(lhs.into(), rhs.into()),
            ElysianExpr::Or(lhs, rhs) => Expr::Or(lhs.into(), rhs.into()),
            ElysianExpr::Dot(lhs, rhs) => Expr::Dot(lhs.into(), rhs.into()),
            ElysianExpr::Mix(lhs, rhs, t) => Expr::Mix(lhs.into(), rhs.into(), t.into()),
            ElysianExpr::Clamp(t, min, max) => Expr::Clamp(t.into(), min.into(), max.into()),
        }
    }
}

impl From<Box<ElysianExpr>> for Box<Expr> {
    fn from(value: Box<ElysianExpr>) -> Self {
        Expr::from(*value).box_expr()
    }
}

pub type BoxExpr = Box<Expr>;

impl core::ops::Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Add(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Sub(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mul(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Div(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::Rem for Expr {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Mod(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::BitAnd for Expr {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        And(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::BitOr for Expr {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Or(self.box_expr(), rhs.box_expr())
    }
}

impl core::ops::Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Neg(self.box_expr())
    }
}

impl Expr {
    pub fn ty(
        &self,
        function_defs: &Vec<FunctionDefinition>,
        types: &IndexMap<PropertyIdentifier, Type>,
    ) -> Type {
        match self {
            Literal(v) => match v {
                Value::Boolean(_) => Type::Boolean,
                Value::Number(n) => Type::Number(match n {
                    super::Number::UInt(_) => NumericType::UInt,
                    super::Number::SInt(_) => NumericType::SInt,
                    super::Number::Float(_) => NumericType::Float,
                }),
                Value::Struct(s) => Type::Struct(s.id.clone()),
            },
            Struct(def, _) => Type::Struct(def.clone()),
            Read(path) => path
                .last()
                .map(|id| types.get(id))
                .flatten()
                .unwrap()
                .clone(),
            Call { function, .. } => types
                .get(
                    &function_defs
                        .iter()
                        .find(|cand| cand.id == *function)
                        .unwrap()
                        .output,
                )
                .unwrap()
                .clone(),
            Neg(t) | Abs(t) | Sign(t) | Round(t) | Sin(t) | Cos(t) | Tan(t) | Asin(t) | Acos(t)
            | Atan(t) => t.ty(function_defs, types),
            Length(t) => match t.ty(function_defs, types) {
                Type::Boolean => panic!("Invalid Length"),
                Type::Number(n) => Type::Number(n),
                Type::Struct(s) => match s.name() {
                    "Vector2" => Type::Number(NumericType::Float),
                    "Vector3" => Type::Number(NumericType::Float),
                    "Vector4" => Type::Number(NumericType::Float),
                    _ => panic!("Invalid Length"),
                },
            },
            Normalize(t) => t.ty(function_defs, types),
            Clamp(t, _, _) => t.ty(function_defs, types),
            Dot(_, _) => Type::Number(NumericType::Float),
            Add(lhs, rhs)
            | Sub(lhs, rhs)
            | Mul(lhs, rhs)
            | Div(lhs, rhs)
            | Mod(lhs, rhs)
            | Min(lhs, rhs)
            | Max(lhs, rhs)
            | Eq(lhs, rhs)
            | Ne(lhs, rhs)
            | Lt(lhs, rhs)
            | Gt(lhs, rhs)
            | And(lhs, rhs)
            | Or(lhs, rhs)
            | Atan2(lhs, rhs)
            | Mix(lhs, rhs, ..) => {
                match (lhs.ty(function_defs, types), rhs.ty(function_defs, types)) {
                    (Type::Boolean, Type::Boolean) => Type::Boolean,
                    (Type::Number(a), Type::Number(b)) => {
                        if a.name() != b.name() {
                            panic!("Invalid Binary Op")
                        }

                        Type::Number(a)
                    }
                    (Type::Number(..), Type::Struct(s)) => Type::Struct(s),
                    (Type::Struct(s), Type::Number(..)) => Type::Struct(s),
                    (Type::Struct(a), Type::Struct(b)) => match self {
                        Add(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2") => Type::Struct(a),
                            ("Vector3", "Vector3") => Type::Struct(a),
                            ("Vector4", "Vector4") => Type::Struct(a),
                            ("Matrix2", "Matrix2") => Type::Struct(a),
                            ("Matrix3", "Matrix3") => Type::Struct(a),
                            ("Matrix4", "Matrix4") => Type::Struct(a),
                            _ => panic!("Invalid Binary Op"),
                        },
                        Sub(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2") => Type::Struct(a),
                            ("Vector3", "Vector3") => Type::Struct(a),
                            ("Vector4", "Vector4") => Type::Struct(a),
                            ("Matrix2", "Matrix2") => Type::Struct(a),
                            ("Matrix3", "Matrix3") => Type::Struct(a),
                            ("Matrix4", "Matrix4") => Type::Struct(a),
                            _ => panic!("Invalid Binary Op"),
                        },
                        Mul(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2") => Type::Struct(a),
                            ("Vector3", "Vector3") => Type::Struct(a),
                            ("Vector4", "Vector4") => Type::Struct(a),
                            ("Matrix2", "Matrix2") => Type::Struct(a),
                            ("Matrix3", "Matrix3") => Type::Struct(a),
                            ("Matrix4", "Matrix4") => Type::Struct(a),
                            ("Vector2", "Matrix2") => Type::Struct(a),
                            ("Matrix2", "Vector2") => Type::Struct(b),
                            ("Vector3", "Matrix3") => Type::Struct(a),
                            ("Matrix3", "Vector3") => Type::Struct(b),
                            ("Vector4", "Matrix4") => Type::Struct(a),
                            ("Matrix4", "Vector4") => Type::Struct(b),
                            _ => panic!("Invalid Binary Op"),
                        },
                        Div(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2") => Type::Struct(a),
                            ("Vector3", "Vector3") => Type::Struct(a),
                            ("Vector4", "Vector4") => Type::Struct(a),
                            _ => panic!("Invalid Binary Op"),
                        },
                        Mod(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2") => Type::Struct(a),
                            ("Vector3", "Vector3") => Type::Struct(a),
                            ("Vector4", "Vector4") => Type::Struct(a),
                            _ => panic!("Invalid Binary Op"),
                        },
                        Lt(_, _)
                        | Gt(_, _)
                        | Eq(_, _)
                        | Ne(_, _)
                        | Dot(_, _)
                        | Min(_, _)
                        | Max(_, _)
                        | Mix(_, _, _) => {
                            if a.name_unique() != b.name_unique() {
                                panic!("Invalid Binary Op")
                            }

                            Type::Struct(a)
                        }
                        And(_, _) | Or(_, _) => {
                            if a.name() == "Bool" && b.name() == "Bool" {
                                Type::Boolean
                            } else {
                                panic!("Invalid Binary Op")
                            }
                        }
                        op => unreachable!("{op:#?}"),
                    },
                    _ => panic!("Invalid Binary Op"),
                }
            }
        }
    }

    pub fn eq(self, rhs: Expr) -> Expr {
        Eq(self.box_expr(), rhs.box_expr())
    }

    pub fn ne(self, rhs: Expr) -> Expr {
        Ne(self.box_expr(), rhs.box_expr())
    }

    pub fn lt(self, rhs: Expr) -> Expr {
        Lt(self.box_expr(), rhs.box_expr())
    }

    pub fn gt(self, rhs: Expr) -> Expr {
        Gt(self.box_expr(), rhs.box_expr())
    }

    pub fn and(self, rhs: Expr) -> Expr {
        And(self.box_expr(), rhs.box_expr())
    }

    pub fn or(self, rhs: Expr) -> Expr {
        Or(self.box_expr(), rhs.box_expr())
    }

    pub fn min(self, rhs: Expr) -> Expr {
        Min(self.box_expr(), rhs.box_expr())
    }

    pub fn max(self, rhs: Expr) -> Expr {
        Max(self.box_expr(), rhs.box_expr())
    }

    pub fn mix(self, rhs: Expr, t: Expr) -> Expr {
        Mix(self.box_expr(), rhs.box_expr(), t.box_expr())
    }

    pub fn clamp(self, min: Expr, max: Expr) -> Expr {
        Clamp(self.box_expr(), min.box_expr(), max.box_expr())
    }

    pub fn dot(self, rhs: Expr) -> Expr {
        Dot(self.box_expr(), rhs.box_expr())
    }

    pub fn atan2(self, rhs: Expr) -> Expr {
        Atan2(self.box_expr(), rhs.box_expr())
    }

    pub fn abs(self) -> Expr {
        Abs(self.box_expr())
    }

    pub fn sign(self) -> Expr {
        Sign(self.box_expr())
    }

    pub fn round(self) -> Expr {
        Round(self.box_expr())
    }

    pub fn length(self) -> Expr {
        Length(self.box_expr())
    }

    pub fn normalize(self) -> Expr {
        Normalize(self.box_expr())
    }

    pub fn sin(self) -> Expr {
        Expr::Sin(self.box_expr())
    }

    pub fn cos(self) -> Expr {
        Expr::Cos(self.box_expr())
    }

    pub fn tan(self) -> Expr {
        Expr::Tan(self.box_expr())
    }

    pub fn asin(self) -> Expr {
        Expr::Asin(self.box_expr())
    }

    pub fn acos(self) -> Expr {
        Expr::Acos(self.box_expr())
    }

    pub fn atan(self) -> Expr {
        Expr::Atan(self.box_expr())
    }

    pub fn output(self) -> Stmt {
        Stmt::Output(self)
    }

    pub fn box_expr(self) -> Box<Expr> {
        Box::new(self)
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
