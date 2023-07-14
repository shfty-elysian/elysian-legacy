use std::marker::PhantomData;

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::elysian::{
    combinator::{Blend, Combinator},
    expr::Expr,
    value::Value,
    Elysian, Field, Modifier,
};

pub trait ToGlam<const N: usize> {
    type Output;

    fn to_glam(self) -> Self::Output;
}

impl<const D: usize, T> ToGlam<D> for Box<T>
where
    T: ToGlam<D>,
{
    type Output = Box<<T as ToGlam<D>>::Output>;

    fn to_glam(self) -> Self::Output {
        Box::new((*self).to_glam())
    }
}

impl ToGlam<1> for Elysian<f32, f32> {
    type Output = Self;

    fn to_glam(self) -> Self::Output {
        self
    }
}

impl ToGlam<2> for Field<f32, [f32; 2]> {
    type Output = Field<f32, Vec2>;

    fn to_glam(self) -> Self::Output {
        match self {
            Field::Point => Field::Point,
            Field::_Phantom(_) => Field::_Phantom(PhantomData),
        }
    }
}

impl ToGlam<2> for Field<f32, [f32; 3]> {
    type Output = Field<f32, Vec3>;

    fn to_glam(self) -> Self::Output {
        match self {
            Field::Point => Field::Point,
            Field::_Phantom(_) => Field::_Phantom(PhantomData),
        }
    }
}

impl ToGlam<2> for Modifier<f32, [f32; 2]> {
    type Output = Modifier<f32, Vec2>;

    fn to_glam(self) -> Self::Output {
        match self {
            Modifier::Translate { delta, shape } => Modifier::Translate {
                delta: delta.to_glam(),
                shape: shape.to_glam(),
            },
            Modifier::Elongate {
                dir,
                infinite,
                shape,
            } => Modifier::Elongate {
                dir: dir.to_glam(),
                infinite,
                shape: shape.to_glam(),
            },
            Modifier::Isosurface { dist, shape } => Modifier::Isosurface {
                dist: dist.to_glam(),
                shape: shape.to_glam(),
            },
            Modifier::Manifold { shape } => Modifier::Manifold {
                shape: shape.to_glam(),
            },
        }
    }
}

impl ToGlam<3> for Modifier<f32, [f32; 3]> {
    type Output = Modifier<f32, Vec3>;

    fn to_glam(self) -> Self::Output {
        match self {
            Modifier::Translate { delta, shape } => Modifier::Translate {
                delta: delta.to_glam(),
                shape: shape.to_glam(),
            },
            Modifier::Elongate {
                dir,
                infinite,
                shape,
            } => Modifier::Elongate {
                dir: dir.to_glam(),
                infinite,
                shape: shape.to_glam(),
            },
            Modifier::Isosurface { dist, shape } => Modifier::Isosurface {
                dist: dist.to_glam(),
                shape: shape.to_glam(),
            },
            Modifier::Manifold { shape } => Modifier::Manifold {
                shape: shape.to_glam(),
            },
        }
    }
}

impl ToGlam<2> for Elysian<f32, [f32; 2]> {
    type Output = Elysian<f32, Vec2>;

    fn to_glam(self) -> Self::Output {
        match self {
            Elysian::Field(field) => Elysian::Field(field.to_glam()),
            Elysian::Modifier(modifier) => Elysian::Modifier(modifier.to_glam()),
            Elysian::Combine {
                combinator,
                shapes: list,
            } => Elysian::Combine {
                combinator: combinator.into_iter().map(ToGlam::to_glam).collect(),
                shapes: list.into_iter().map(ToGlam::to_glam).collect(),
            },
            Elysian::Alias(_) => panic!("Aliases must be expanded before ToGlam"),
        }
    }
}

impl ToGlam<3> for Elysian<f32, [f32; 3]> {
    type Output = Elysian<f32, Vec3>;

    fn to_glam(self) -> Self::Output {
        match self {
            Elysian::Field(f) => Elysian::Field(f.to_glam()),
            Elysian::Modifier(m) => Elysian::Modifier(m.to_glam()),
            Elysian::Combine {
                combinator,
                shapes: list,
            } => Elysian::Combine {
                combinator: combinator.into_iter().map(ToGlam::to_glam).collect(),
                shapes: list.into_iter().map(ToGlam::to_glam).collect(),
            },
            Elysian::Alias(_) => panic!("Aliases must be expanded before ToGlam"),
        }
    }
}

impl<const N: usize> ToGlam<N> for f32 {
    type Output = f32;

    fn to_glam(self) -> Self::Output {
        self
    }
}

impl ToGlam<2> for [f32; 2] {
    type Output = Vec2;

    fn to_glam(self) -> Self::Output {
        Vec2::new(self[0], self[1])
    }
}

impl ToGlam<3> for [f32; 3] {
    type Output = Vec3;

    fn to_glam(self) -> Self::Output {
        Vec3::new(self[0], self[1], self[2])
    }
}

impl<const D: usize, N, V> ToGlam<D> for Expr<N, V>
where
    N: ToGlam<D>,
    V: ToGlam<D>,
{
    type Output = Expr<<N as ToGlam<D>>::Output, <V as ToGlam<D>>::Output>;

    fn to_glam(self) -> Self::Output {
        match self {
            Expr::Literal(v) => Expr::Literal(v.to_glam()),
            Expr::Read(a) => Expr::Read(a),
            Expr::Add(lhs, rhs) => Expr::Add(lhs.to_glam(), rhs.to_glam()),
            Expr::Sub(lhs, rhs) => Expr::Sub(lhs.to_glam(), rhs.to_glam()),
            Expr::Mul(lhs, rhs) => Expr::Mul(lhs.to_glam(), rhs.to_glam()),
            Expr::Div(lhs, rhs) => Expr::Div(lhs.to_glam(), rhs.to_glam()),
            Expr::Min(lhs, rhs) => Expr::Min(lhs.to_glam(), rhs.to_glam()),
            Expr::Max(lhs, rhs) => Expr::Max(lhs.to_glam(), rhs.to_glam()),
            Expr::Mix(lhs, rhs, t) => Expr::Mix(lhs.to_glam(), rhs.to_glam(), t.to_glam()),
            Expr::Lt(lhs, rhs) => Expr::Lt(lhs.to_glam(), rhs.to_glam()),
            Expr::Gt(lhs, rhs) => Expr::Gt(lhs.to_glam(), rhs.to_glam()),
            Expr::Neg(t) => Expr::Neg(t.to_glam()),
            Expr::Abs(t) => Expr::Abs(t.to_glam()),
            Expr::Sign(t) => Expr::Sign(t.to_glam()),
            Expr::Length(t) => Expr::Length(t.to_glam()),
            Expr::Normalize(t) => Expr::Normalize(t.to_glam()),
            Expr::Dot(lhs, rhs) => Expr::Dot(lhs.to_glam(), rhs.to_glam()),
        }
    }
}

impl<const D: usize, N, V> ToGlam<D> for Combinator<N, V>
where
    N: ToGlam<D>,
    V: ToGlam<D>,
{
    type Output = Combinator<<N as ToGlam<D>>::Output, <V as ToGlam<D>>::Output>;

    fn to_glam(self) -> Self::Output {
        match self {
            Combinator::Boolean(b) => Combinator::Boolean(b),
            Combinator::Blend(b) => Combinator::Blend(b.to_glam()),
        }
    }
}

impl<const D: usize, N, V> ToGlam<D> for Blend<N, V>
where
    N: ToGlam<D>,
    V: ToGlam<D>,
{
    type Output = Blend<<N as ToGlam<D>>::Output, <V as ToGlam<D>>::Output>;

    fn to_glam(self) -> Self::Output {
        match self {
            Blend::SmoothUnion { attr, k } => Blend::SmoothUnion {
                attr,
                k: k.to_glam(),
            },
            Blend::SmoothIntersection { attr, k } => Blend::SmoothIntersection {
                attr,
                k: k.to_glam(),
            },
            Blend::SmoothSubtraction { attr, k } => Blend::SmoothSubtraction {
                attr,
                k: k.to_glam(),
            },
        }
    }
}

impl<const D: usize, N, V> ToGlam<D> for Value<N, V>
where
    N: ToGlam<D>,
    V: ToGlam<D>,
{
    type Output = Value<<N as ToGlam<D>>::Output, <V as ToGlam<D>>::Output>;

    fn to_glam(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(n.to_glam()),
            Value::Vector(v) => Value::Vector(v.to_glam()),
        }
    }
}
