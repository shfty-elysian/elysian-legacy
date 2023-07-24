use rust_gpu_bridge::{
    glam::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4},
    Abs, Dot, Length, Max, Min, Mix, Normalize, Sign,
};
use tracing::instrument;

use crate::ir::{
    ast::{Property, Value},
    module::StructDefinition,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::{
    Number, MATRIX2_STRUCT, MATRIX3_STRUCT, MATRIX4_STRUCT, VECTOR2_STRUCT, VECTOR3_STRUCT,
    VECTOR4_STRUCT, W, W_AXIS_4, X, X_AXIS_2, X_AXIS_3, X_AXIS_4, Y, Y_AXIS_2, Y_AXIS_3, Y_AXIS_4,
    Z, Z_AXIS_3, Z_AXIS_4,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct Struct {
    pub def: &'static StructDefinition,
    pub members: BTreeMap<Property, Value>,
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (prop, val) in &self.members {
            write!(f, "{prop:}: {val:}")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Struct {
    pub fn new(def: &'static StructDefinition) -> Self {
        Struct {
            def,
            members: Default::default(),
        }
    }
    pub fn try_get_ref(&self, key: &Property) -> Option<&Value> {
        self.members.get(key)
    }

    pub fn try_get_mut(&mut self, key: &Property) -> Option<&mut Value> {
        self.members.get_mut(key)
    }

    pub fn set_mut(&mut self, key: Property, t: Value) {
        self.members.insert(key, t);
    }

    pub fn get(&self, key: &Property) -> Value {
        self.get_ref(key).clone()
    }

    fn get_ref(&self, key: &Property) -> &Value {
        self.try_get_ref(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    pub fn get_mut(&mut self, key: &Property) -> &mut Value {
        self.try_get_mut(key)
            .unwrap_or_else(|| panic!("Invalid key {key:#?}"))
    }

    pub fn try_get(&self, key: &Property) -> Option<Value> {
        self.try_get_ref(key).cloned()
    }

    pub fn set(mut self, key: Property, t: Value) -> Self
    where
        Self: Sized,
    {
        self.set_mut(key, t);
        self
    }

    #[instrument]
    pub fn remove(&mut self, key: &Property) -> Value {
        self.members
            .remove(key)
            .unwrap_or_else(|| panic!("Invalid key {key:?}"))
    }

    #[instrument]
    pub fn get_context(&self, key: &Property) -> Struct {
        let Value::Struct(c) = self.get_ref(key) else {
        panic!("Value is not a context")
    };

        c.clone()
    }

    #[instrument]
    pub fn set_number(mut self, key: Property, n: Number) -> Self {
        self.members.insert(key, Value::Number(n));
        self
    }
}

impl Add<Number> for Struct {
    type Output = Self;

    fn add(mut self, rhs: Number) -> Self::Output {
        for (_, value) in self.members.iter_mut() {
            *value = value.clone() + rhs.into();
        }
        self
    }
}

impl Sub<Number> for Struct {
    type Output = Self;

    fn sub(mut self, rhs: Number) -> Self::Output {
        for (_, value) in self.members.iter_mut() {
            *value = value.clone() - rhs.into();
        }
        self
    }
}

impl Mul<Number> for Struct {
    type Output = Self;

    fn mul(mut self, rhs: Number) -> Self::Output {
        for (_, value) in self.members.iter_mut() {
            *value = value.clone() * rhs.into();
        }
        self
    }
}

impl Div<Number> for Struct {
    type Output = Self;

    fn div(mut self, rhs: Number) -> Self::Output {
        for (_, value) in self.members.iter_mut() {
            *value = value.clone() / rhs.into();
        }
        self
    }
}

impl Add<Struct> for Struct {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => (Vec2::from(self) + Vec2::from(rhs)).into(),
            "Vector3" => (Vec3::from(self) + Vec3::from(rhs)).into(),
            "Vector4" => (Vec4::from(self) + Vec4::from(rhs)).into(),
            _ => panic!("Can't Add an arbitrary struct"),
        }
    }
}

impl Sub<Struct> for Struct {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => (Vec2::from(self) - Vec2::from(rhs)).into(),
            "Vector3" => (Vec3::from(self) - Vec3::from(rhs)).into(),
            "Vector4" => (Vec4::from(self) - Vec4::from(rhs)).into(),
            _ => panic!("Can't Sub an arbitrary struct"),
        }
    }
}

impl Mul<Struct> for Struct {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.def.name(), rhs.def.name()) {
            ("Vector2", "Vector2") => (Vec2::from(self) * Vec2::from(rhs)).into(),
            ("Vector3", "Vector3") => (Vec3::from(self) * Vec3::from(rhs)).into(),
            ("Vector4", "Vector4") => (Vec4::from(self) * Vec4::from(rhs)).into(),
            ("Matrix2", "Matrix2") => (Mat2::from(self) * Mat2::from(rhs)).into(),
            ("Matrix3", "Matrix3") => (Mat3::from(self) * Mat3::from(rhs)).into(),
            ("Matrix4", "Matrix4") => (Mat4::from(self) * Mat4::from(rhs)).into(),
            ("Matrix2", "Vector2") => (Mat2::from(self) * Vec2::from(rhs)).into(),
            ("Matrix3", "Vector3") => (Mat3::from(self) * Vec3::from(rhs)).into(),
            ("Matrix4", "Vector4") => (Mat4::from(self) * Vec4::from(rhs)).into(),
            _ => panic!("Can't Mul an arbitrary struct"),
        }
    }
}

impl Div<Struct> for Struct {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => (Vec2::from(self) + Vec2::from(rhs)).into(),
            "Vector3" => (Vec3::from(self) + Vec3::from(rhs)).into(),
            "Vector4" => (Vec4::from(self) + Vec4::from(rhs)).into(),
            _ => panic!("Can't Div an arbitrary struct"),
        }
    }
}

impl Dot for Struct {
    type Output = Number;

    fn dot(self, rhs: Self) -> Self::Output {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => Vec2::from(self).dot(Vec2::from(rhs)).into(),
            "Vector3" => Vec3::from(self).dot(Vec3::from(rhs)).into(),
            "Vector4" => Vec4::from(self).dot(Vec4::from(rhs)).into(),
            _ => panic!("Can't Dot an arbitrary struct"),
        }
    }
}

impl Min for Struct {
    fn min(self, rhs: Self) -> Self {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => Vec2::from(self).min(Vec2::from(rhs)).into(),
            "Vector3" => Vec3::from(self).min(Vec3::from(rhs)).into(),
            "Vector4" => Vec4::from(self).min(Vec4::from(rhs)).into(),
            _ => panic!("Can't Min an arbitrary struct"),
        }
    }
}

impl Max for Struct {
    fn max(self, rhs: Self) -> Self {
        assert!(self.def == rhs.def);
        match self.def.name() {
            "Vector2" => Vec2::from(self).max(Vec2::from(rhs)).into(),
            "Vector3" => Vec3::from(self).max(Vec3::from(rhs)).into(),
            "Vector4" => Vec4::from(self).max(Vec4::from(rhs)).into(),
            _ => panic!("Can't Max an arbitrary struct"),
        }
    }
}

impl Mix for Struct {
    type T = Number;

    fn mix(self, to: Self, t: Self::T) -> Self {
        assert!(self.def == to.def);
        match self.def.name() {
            "Vector2" => Vec2::from(self).mix(Vec2::from(to), t.into()).into(),
            "Vector3" => Vec3::from(self).mix(Vec3::from(to), t.into()).into(),
            "Vector4" => Vec4::from(self).mix(Vec4::from(to), t.into()).into(),
            _ => panic!("Can't Mix an arbitrary struct"),
        }
    }
}

impl Neg for Struct {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.def.name() {
            "Vector2" => Vec2::from(self).neg().into(),
            "Vector3" => Vec3::from(self).neg().into(),
            "Vector4" => Vec4::from(self).neg().into(),
            _ => panic!("Can't Neg an arbitrary struct"),
        }
    }
}

impl Abs for Struct {
    fn abs(self) -> Self {
        match self.def.name() {
            "Vector2" => Vec2::from(self).abs().into(),
            "Vector3" => Vec3::from(self).abs().into(),
            "Vector4" => Vec4::from(self).abs().into(),
            _ => panic!("Can't Abs an arbitrary struct"),
        }
    }
}

impl Sign for Struct {
    fn sign(self) -> Self {
        match self.def.name() {
            "Vector2" => Vec2::from(self).sign().into(),
            "Vector3" => Vec3::from(self).sign().into(),
            "Vector4" => Vec4::from(self).sign().into(),
            _ => panic!("Can't Sign an arbitrary struct"),
        }
    }
}

impl Length for Struct {
    type Output = Number;

    fn length(self) -> Self::Output {
        match self.def.name() {
            "Vector2" => Vec2::from(self).length().into(),
            "Vector3" => Vec3::from(self).length().into(),
            "Vector4" => Vec4::from(self).length().into(),
            _ => panic!("Can't Normalize an arbitrary struct"),
        }
    }
}

impl Normalize for Struct {
    fn normalize(self) -> Self {
        match self.def.name() {
            "Vector2" => Vec2::from(self).normalize_or_zero().into(),
            "Vector3" => Vec3::from(self).normalize_or_zero().into(),
            "Vector4" => Vec4::from(self).normalize_or_zero().into(),
            _ => panic!("Can't Normalize an arbitrary struct"),
        }
    }
}

impl From<Struct> for Vec2 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Vector2" => Vec2::new(value.get(&X).into(), value.get(&Y).into()),
            _ => panic!("Struct is not a Vec2"),
        }
    }
}

impl From<Struct> for Vec3 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Vector3" => Vec3::new(
                value.get(&X).into(),
                value.get(&Y).into(),
                value.get(&Z).into(),
            ),
            _ => panic!("Struct is not a Vec3"),
        }
    }
}

impl From<Struct> for Vec4 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Vector4" => Vec4::new(
                value.get(&X).into(),
                value.get(&Y).into(),
                value.get(&Z).into(),
                value.get(&W).into(),
            ),
            _ => panic!("Struct is not a Vec3"),
        }
    }
}

impl From<Struct> for Mat2 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Matrix2" => Mat2::from_cols(value.get(&X_AXIS_2).into(), value.get(&Y_AXIS_2).into()),
            _ => panic!("Struct is not a Mat2"),
        }
    }
}

impl From<Struct> for Mat3 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Matrix3" => Mat3::from_cols(
                value.get(&X_AXIS_3).into(),
                value.get(&Y_AXIS_3).into(),
                value.get(&Z_AXIS_3).into(),
            ),
            _ => panic!("Struct is not a Mat3"),
        }
    }
}

impl From<Struct> for Mat4 {
    fn from(value: Struct) -> Self {
        match value.def.name() {
            "Matrix4" => Mat4::from_cols(
                value.get(&X_AXIS_4).into(),
                value.get(&Y_AXIS_4).into(),
                value.get(&Z_AXIS_4).into(),
                value.get(&W_AXIS_4).into(),
            ),
            _ => panic!("Struct is not a Mat4"),
        }
    }
}

impl From<Vec2> for Struct {
    fn from(value: Vec2) -> Self {
        Struct::new(VECTOR2_STRUCT)
            .set(X, value.x.into())
            .set(Y, value.y.into())
    }
}

impl From<Vec3> for Struct {
    fn from(value: Vec3) -> Self {
        Struct::new(VECTOR3_STRUCT)
            .set(X, value.x.into())
            .set(Y, value.y.into())
            .set(Z, value.z.into())
    }
}

impl From<Vec4> for Struct {
    fn from(value: Vec4) -> Self {
        Struct::new(VECTOR4_STRUCT)
            .set(X, value.x.into())
            .set(Y, value.y.into())
            .set(Z, value.z.into())
            .set(W, value.w.into())
    }
}

impl From<Mat2> for Struct {
    fn from(value: Mat2) -> Self {
        Struct::new(MATRIX2_STRUCT)
            .set(X_AXIS_2, value.x_axis.into())
            .set(Y_AXIS_2, value.y_axis.into())
    }
}

impl From<Mat3> for Struct {
    fn from(value: Mat3) -> Self {
        Struct::new(MATRIX3_STRUCT)
            .set(X_AXIS_3, value.x_axis.into())
            .set(Y_AXIS_3, value.y_axis.into())
            .set(Z_AXIS_3, value.z_axis.into())
    }
}

impl From<Mat4> for Struct {
    fn from(value: Mat4) -> Self {
        Struct::new(MATRIX4_STRUCT)
            .set(X_AXIS_4, value.x_axis.into())
            .set(Y_AXIS_4, value.y_axis.into())
            .set(Z_AXIS_4, value.z_axis.into())
            .set(W_AXIS_4, value.w_axis.into())
    }
}
