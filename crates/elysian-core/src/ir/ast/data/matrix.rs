use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use rust_gpu_bridge::glam::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

use crate::ir::ast::Number;

use super::Vector;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Matrix {
    Matrix2(Vector, Vector),
    Matrix3(Vector, Vector, Vector),
    Matrix4(Vector, Vector, Vector, Vector),
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Matrix::Matrix2(x, y) => write!(f, "({x:}, {y:})"),
            Matrix::Matrix3(x, y, z) => write!(f, "({x:}, {y:}, {z:})"),
            Matrix::Matrix4(x, y, z, w) => write!(f, "({x:}, {y:}, {z:}, {w:})"),
        }
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        match (self, rhs) {
            (a @ Matrix::Matrix2(_, _), b @ Matrix::Matrix2(_, _)) => {
                (Mat2::from(a) + Mat2::from(b)).into()
            }
            (a @ Matrix::Matrix3(_, _, _), b @ Matrix::Matrix3(_, _, _)) => {
                (Mat3::from(a) + Mat3::from(b)).into()
            }
            (a @ Matrix::Matrix4(_, _, _, _), b @ Matrix::Matrix4(_, _, _, _)) => {
                (Mat4::from(a) + Mat4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        match (self, rhs) {
            (a @ Matrix::Matrix2(_, _), b @ Matrix::Matrix2(_, _)) => {
                (Mat2::from(a) - Mat2::from(b)).into()
            }
            (a @ Matrix::Matrix3(_, _, _), b @ Matrix::Matrix3(_, _, _)) => {
                (Mat3::from(a) - Mat3::from(b)).into()
            }
            (a @ Matrix::Matrix4(_, _, _, _), b @ Matrix::Matrix4(_, _, _, _)) => {
                (Mat4::from(a) - Mat4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        match (self, rhs) {
            (a @ Matrix::Matrix2(..), b @ Matrix::Matrix2(..)) => {
                (Mat2::from(a) * Mat2::from(b)).into()
            }
            (a @ Matrix::Matrix3(..), b @ Matrix::Matrix3(..)) => {
                (Mat3::from(a) * Mat3::from(b)).into()
            }
            (a @ Matrix::Matrix4(..), b @ Matrix::Matrix4(..)) => {
                (Mat4::from(a) * Mat4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        match (self, rhs) {
            (a @ Matrix::Matrix2(..), b @ Vector::Vector2(..)) => {
                (Mat2::from(a) * Vec2::from(b)).into()
            }
            (a @ Matrix::Matrix3(..), b @ Vector::Vector3(..)) => {
                (Mat3::from(a) * Vec3::from(b)).into()
            }
            (a @ Matrix::Matrix4(..), b @ Vector::Vector4(..)) => {
                (Mat4::from(a) * Vec4::from(b)).into()
            }
            _ => panic!("Invalid Mul"),
        }
    }
}

impl Mul<Number> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (a @ Matrix::Matrix2(..), b @ Number::Float(..)) => {
                (Mat2::from(a) * f32::from(b)).into()
            }
            (a @ Matrix::Matrix3(..), b @ Number::Float(..)) => {
                (Mat3::from(a) * f32::from(b)).into()
            }
            (a @ Matrix::Matrix4(..), b @ Number::Float(..)) => {
                (Mat4::from(a) * f32::from(b)).into()
            }
            _ => panic!("Invalid Mul"),
        }
    }
}

impl From<Matrix> for Mat2 {
    fn from(value: Matrix) -> Self {
        match value {
            Matrix::Matrix2(x, y) => Mat2::from_cols(x.into(), y.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Matrix> for Mat3 {
    fn from(value: Matrix) -> Self {
        match value {
            Matrix::Matrix3(x, y, z) => Mat3::from_cols(x.into(), y.into(), z.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Matrix> for Mat4 {
    fn from(value: Matrix) -> Self {
        match value {
            Matrix::Matrix4(x, y, z, w) => Mat4::from_cols(x.into(), y.into(), z.into(), w.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Mat2> for Matrix {
    fn from(value: Mat2) -> Self {
        Matrix::Matrix2(value.x_axis.into(), value.y_axis.into())
    }
}

impl From<Mat3> for Matrix {
    fn from(value: Mat3) -> Self {
        Matrix::Matrix3(
            value.x_axis.into(),
            value.y_axis.into(),
            value.z_axis.into(),
        )
    }
}

impl From<Mat4> for Matrix {
    fn from(value: Mat4) -> Self {
        Matrix::Matrix4(
            value.x_axis.into(),
            value.y_axis.into(),
            value.z_axis.into(),
            value.w_axis.into(),
        )
    }
}
