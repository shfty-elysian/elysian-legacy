use std::fmt::Debug;

use rust_gpu_bridge::glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};

use super::{Number, Vector2, VectorSpace};

pub trait TypeSpec:
    'static + VectorSpace<1> + VectorSpace<2> + VectorSpace<3> + VectorSpace<4>
{
    type NUMBER: 'static + Debug + Clone + PartialEq + PartialOrd + Number<Self>;
    type VECTOR2: 'static + Debug + Clone + PartialEq + Vector2<Self>;
    type VECTOR3: 'static + Debug + Clone + PartialEq;
    type VECTOR4: 'static + Debug + Clone + PartialEq;
}

/*
pub enum GlamI32 {}

impl TypeSpec for GlamI32 {
    type NUMBER = i32;
    type VECTOR2 = IVec2;
    type VECTOR3 = IVec3;
    type VECTOR4 = IVec4;
}
*/

pub enum GlamF32 {}

impl TypeSpec for GlamF32 {
    type NUMBER = f32;
    type VECTOR2 = Vec2;
    type VECTOR3 = Vec3;
    type VECTOR4 = Vec4;
}

pub enum GlamF64 {}

impl TypeSpec for GlamF64 {
    type NUMBER = f64;
    type VECTOR2 = DVec2;
    type VECTOR3 = DVec3;
    type VECTOR4 = DVec4;
}

/*
pub enum PrimI32 {}

impl TypeSpec for PrimI32 {
    type NUMBER = i32;
    type VECTOR2 = [i32; 2];
    type VECTOR3 = [i32; 3];
    type VECTOR4 = [i32; 4];
}

pub enum PrimF32 {}

impl TypeSpec for PrimF32 {
    type NUMBER = f32;
    type VECTOR2 = [f32; 2];
    type VECTOR3 = [f32; 3];
    type VECTOR4 = [f32; 4];
}

pub enum PrimF64 {}

impl TypeSpec for PrimF64 {
    type NUMBER = f64;
    type VECTOR2 = [f64; 2];
    type VECTOR3 = [f64; 3];
    type VECTOR4 = [f64; 4];
}
*/
