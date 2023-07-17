use rust_gpu_bridge::glam::{Vec2, Vec3};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Value<N, V> {
    Number(N),
    Vector(V),
}

impl<N, V> std::hash::Hash for Value<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait IntoValue<N, V> {
    fn value(self) -> Value<N, V>;
}

impl<N, V> IntoValue<N, V> for Value<N, V> {
    fn value(self) -> Value<N, V> {
        self
    }
}

impl<V> IntoValue<f32, V> for f32 {
    fn value(self) -> Value<f32, V> {
        Value::Number(self)
    }
}

impl<N> IntoValue<N, [f32; 2]> for [f32; 2] {
    fn value(self) -> Value<N, [f32; 2]> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, [f32; 3]> for [f32; 3] {
    fn value(self) -> Value<N, [f32; 3]> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, Vec2> for Vec2 {
    fn value(self) -> Value<N, Vec2> {
        Value::Vector(self)
    }
}

impl<N> IntoValue<N, Vec3> for Vec3 {
    fn value(self) -> Value<N, Vec3> {
        Value::Vector(self)
    }
}
