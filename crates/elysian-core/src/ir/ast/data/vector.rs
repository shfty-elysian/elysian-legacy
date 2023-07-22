use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec4},
    Abs, Dot, Length, Max, Min, Mix, Normalize, Sign,
};

use crate::ir::{
    ast::{Identifier, Number, Property},
    module::{FieldDefinition, StructDefinition, Type},
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Vector {
    Vector2(Number, Number),
    Vector3(Number, Number, Number),
    Vector4(Number, Number, Number, Number),
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::Vector2(x, y) => write!(f, "({x:}, {y:})"),
            Vector::Vector3(x, y, z) => write!(f, "({x:}, {y:}, {z:})"),
            Vector::Vector4(x, y, z, w) => write!(f, "({x:}, {y:}, {z:}, {w:})"),
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                (Vec2::from(a) + Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                (Vec3::from(a) + Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                (Vec4::from(a) + Vec4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                (Vec2::from(a) - Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                (Vec3::from(a) - Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                (Vec4::from(a) - Vec4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                (Vec2::from(a) * Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                (Vec3::from(a) * Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                (Vec4::from(a) * Vec4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                (Vec2::from(a) / Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                (Vec3::from(a) / Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                (Vec4::from(a) / Vec4::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Add<Number> for Vector {
    type Output = Vector;

    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Number::Float(_)) => {
                (Vec2::from(a) + f32::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Number::Float(_)) => {
                (Vec3::from(a) + f32::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Number::Float(_)) => {
                (Vec4::from(a) + f32::from(b)).into()
            }
            _ => panic!("Invalid Add"),
        }
    }
}

impl Sub<Number> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Number::Float(_)) => {
                (Vec2::from(a) - f32::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Number::Float(_)) => {
                (Vec3::from(a) - f32::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Number::Float(_)) => {
                (Vec4::from(a) - f32::from(b)).into()
            }
            _ => panic!("Invalid Sub"),
        }
    }
}

impl Mul<Number> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Number::Float(_)) => {
                (Vec2::from(a) * f32::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Number::Float(_)) => {
                (Vec3::from(a) * f32::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Number::Float(_)) => {
                (Vec4::from(a) * f32::from(b)).into()
            }
            _ => panic!("Invalid Mul"),
        }
    }
}

impl Div<Number> for Vector {
    type Output = Vector;

    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Number::Float(_)) => {
                (Vec2::from(a) / f32::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Number::Float(_)) => {
                (Vec3::from(a) / f32::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Number::Float(_)) => {
                (Vec4::from(a) / f32::from(b)).into()
            }
            _ => panic!("Invalid Div"),
        }
    }
}

impl Mix for Vector {
    type T = Number;

    fn mix(self, to: Self, t: Self::T) -> Self {
        match (self, to, t) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _), t @ Number::Float(_)) => {
                Vec2::from(a).mix(Vec2::from(b), t.into()).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _), t @ Number::Float(_)) => {
                Vec3::from(a).mix(Vec3::from(b), t.into()).into()
            }
            (
                a @ Vector::Vector4(_, _, _, _),
                b @ Vector::Vector4(_, _, _, _),
                t @ Number::Float(_),
            ) => Vec4::from(a).mix(Vec4::from(b), t.into()).into(),
            _ => panic!("Invalid Mix"),
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            v @ Vector::Vector2(_, _) => (-Vec2::from(v)).into(),
            v @ Vector::Vector3(_, _, _) => (-Vec3::from(v)).into(),
            v @ Vector::Vector4(_, _, _, _) => (-Vec4::from(v)).into(),
        }
    }
}

impl Abs for Vector {
    fn abs(self) -> Self {
        match self {
            v @ Vector::Vector2(_, _) => Vec2::from(v).abs().into(),
            v @ Vector::Vector3(_, _, _) => Vec3::from(v).abs().into(),
            v @ Vector::Vector4(_, _, _, _) => Vec4::from(v).abs().into(),
        }
    }
}

impl Sign for Vector {
    fn sign(self) -> Self {
        match self {
            v @ Vector::Vector2(_, _) => Vec2::from(v).sign().into(),
            v @ Vector::Vector3(_, _, _) => Vec3::from(v).sign().into(),
            v @ Vector::Vector4(_, _, _, _) => Vec4::from(v).sign().into(),
        }
    }
}

impl Min for Vector {
    fn min(self, rhs: Self) -> Self {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                Vec2::from(a).min(Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                Vec3::from(a).min(Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                Vec4::from(a).min(Vec4::from(b)).into()
            }
            _ => panic!("Invalid Min"),
        }
    }
}

impl Max for Vector {
    fn max(self, rhs: Self) -> Self {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                Vec2::from(a).max(Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                Vec3::from(a).max(Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                Vec4::from(a).max(Vec4::from(b)).into()
            }
            _ => panic!("Invalid Max"),
        }
    }
}

impl Length for Vector {
    type Output = Number;

    fn length(self) -> Self::Output {
        match self {
            v @ Vector::Vector2(_, _) => Vec2::from(v).length().into(),
            v @ Vector::Vector3(_, _, _) => Vec3::from(v).length().into(),
            v @ Vector::Vector4(_, _, _, _) => Vec4::from(v).length().into(),
        }
    }
}

impl Normalize for Vector {
    fn normalize(self) -> Self {
        match self {
            v @ Vector::Vector2(_, _) => Vec2::from(v).normalize().into(),
            v @ Vector::Vector3(_, _, _) => Vec3::from(v).normalize().into(),
            v @ Vector::Vector4(_, _, _, _) => Vec4::from(v).normalize().into(),
        }
    }
}

impl Dot for Vector {
    type Output = Number;

    fn dot(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (a @ Vector::Vector2(_, _), b @ Vector::Vector2(_, _)) => {
                Vec2::from(a).dot(Vec2::from(b)).into()
            }
            (a @ Vector::Vector3(_, _, _), b @ Vector::Vector3(_, _, _)) => {
                Vec3::from(a).dot(Vec3::from(b)).into()
            }
            (a @ Vector::Vector4(_, _, _, _), b @ Vector::Vector4(_, _, _, _)) => {
                Vec4::from(a).dot(Vec4::from(b)).into()
            }
            _ => panic!("Invalid Dot"),
        }
    }
}

impl From<Vector> for Vec2 {
    fn from(value: Vector) -> Self {
        match value {
            Vector::Vector2(x, y) => Vec2::new(x.into(), y.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Vector> for Vec3 {
    fn from(value: Vector) -> Self {
        match value {
            Vector::Vector3(x, y, z) => Vec3::new(x.into(), y.into(), z.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Vector> for Vec4 {
    fn from(value: Vector) -> Self {
        match value {
            Vector::Vector4(x, y, z, w) => Vec4::new(x.into(), y.into(), z.into(), w.into()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl From<Vec2> for Vector {
    fn from(value: Vec2) -> Self {
        Vector::Vector2(value.x.into(), value.y.into())
    }
}

impl From<Vec3> for Vector {
    fn from(value: Vec3) -> Self {
        Vector::Vector3(value.x.into(), value.y.into(), value.z.into())
    }
}

impl From<Vec4> for Vector {
    fn from(value: Vec4) -> Self {
        Vector::Vector4(
            value.x.into(),
            value.y.into(),
            value.z.into(),
            value.w.into(),
        )
    }
}

// New Struct-based impl

pub const X: Property = Property::new_primitive("x", Type::Number);
pub const Y: Property = Property::new_primitive("y", Type::Number);
pub const Z: Property = Property::new_primitive("z", Type::Number);
pub const W: Property = Property::new_primitive("w", Type::Number);

pub const VECTOR2_STRUCT: StructDefinition = StructDefinition {
    id: Identifier::new("Vector2", 8915589261187194730),
    public: false,
    fields: &[
        FieldDefinition {
            prop: X,
            public: true,
        },
        FieldDefinition {
            prop: Y,
            public: true,
        },
    ],
};

pub const VECTOR3_STRUCT: StructDefinition = StructDefinition {
    id: Identifier::new("Vector3", 8915589261187194730),
    public: false,
    fields: &[
        FieldDefinition {
            prop: X,
            public: true,
        },
        FieldDefinition {
            prop: Y,
            public: true,
        },
        FieldDefinition {
            prop: Z,
            public: true,
        },
    ],
};

pub const VECTOR4_STRUCT: StructDefinition = StructDefinition {
    id: Identifier::new("Vector4", 8915589261187194730),
    public: false,
    fields: &[
        FieldDefinition {
            prop: X,
            public: true,
        },
        FieldDefinition {
            prop: Y,
            public: true,
        },
        FieldDefinition {
            prop: Z,
            public: true,
        },
        FieldDefinition {
            prop: W,
            public: true,
        },
    ],
};
