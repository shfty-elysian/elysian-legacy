use std::fmt::Debug;

use crate::elysian::{Elysian, Field, PostModifier, PreModifier};

pub trait Expand<N, V>: Debug {
    fn expand(&self) -> Elysian<N, V>;
}

impl<N, V> Expand<N, V> for Elysian<N, V>
where
    N: Debug + Copy,
    V: Debug + Copy,
{
    fn expand(&self) -> Elysian<N, V> {
        match self {
            Elysian::Field(field) => Elysian::Field(match field {
                Field::Point => Field::Point,
                Field::_Phantom(p) => Field::_Phantom(*p),
            }),
            Elysian::PreModifier(modifier) => Elysian::PreModifier(match modifier {
                PreModifier::Translate { delta, shape: ast } => PreModifier::Translate {
                    delta: delta.clone(),
                    shape: Box::new(ast.expand()),
                },
                PreModifier::Elongate {
                    dir,
                    infinite,
                    shape: ast,
                } => PreModifier::Elongate {
                    dir: dir.clone(),
                    infinite: *infinite,
                    shape: Box::new(ast.expand()),
                },
            }),
            Elysian::PostModifier(modifier) => Elysian::PostModifier(match modifier {
                PostModifier::Isosurface { dist, shape: ast } => PostModifier::Isosurface {
                    dist: dist.clone(),
                    shape: Box::new(ast.expand()),
                },
                PostModifier::Manifold { shape: ast } => PostModifier::Manifold {
                    shape: Box::new(ast.expand()),
                },
            }),
            Elysian::Combine {
                combinator,
                shapes: list,
            } => Elysian::Combine {
                combinator: combinator.clone(),
                shapes: list.into_iter().map(Expand::expand).collect(),
            },
            Elysian::Alias(a) => a.expand(),
        }
    }
}

impl<N, V> Expand<N, V> for Box<Elysian<N, V>>
where
    N: Debug + Copy,
    V: Debug + Copy,
{
    fn expand(&self) -> Elysian<N, V> {
        (**self).expand()
    }
}

impl<N, V> Expand<N, V> for Box<dyn Expand<N, V>> {
    fn expand(&self) -> Elysian<N, V> {
        self.as_ref().expand()
    }
}
