use std::fmt::Debug;

use crate::elysian::{Elysian, Field, Modifier};

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
            Elysian::Modifier(modifier) => Elysian::Modifier(match modifier {
                Modifier::Translate { delta, shape: ast } => Modifier::Translate {
                    delta: delta.clone(),
                    shape: Box::new(ast.expand()),
                },
                Modifier::Isosurface { dist, shape: ast } => Modifier::Isosurface {
                    dist: dist.clone(),
                    shape: Box::new(ast.expand()),
                },
                Modifier::Manifold { shape: ast } => Modifier::Manifold {
                    shape: Box::new(ast.expand()),
                },
                Modifier::Elongate {
                    dir,
                    infinite,
                    shape: ast,
                } => Modifier::Elongate {
                    dir: dir.clone(),
                    infinite: *infinite,
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
