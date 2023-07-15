use std::marker::PhantomData;

use self::{combinator::Combinator, expand::Expand, expr::Expr};

pub mod alias;
pub mod attribute;
pub mod combinator;
pub mod expand;
pub mod expr;
pub mod value;

pub type ElysianBox<N, V> = Box<Elysian<N, V>>;
pub type ElysianList<N, V> = Vec<Elysian<N, V>>;

#[macro_export]
macro_rules! list {
    ($($expr:expr),* $(,)?) => {
        vec![$(Box::new($expr)),*]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Field<N, V> {
    Point,
    _Phantom(PhantomData<(N, V)>),
}

impl<N, V> Field<N, V> {
    pub fn field(self) -> Elysian<N, V> {
        Elysian::Field(self)
    }
}

#[derive(Debug)]
pub enum PreModifier<N, V> {
    Translate {
        delta: Expr<N, V>,
        shape: ElysianBox<N, V>,
    },
    Elongate {
        dir: Expr<N, V>,
        infinite: bool,
        shape: ElysianBox<N, V>,
    },
}

impl<N, V> PreModifier<N, V> {
    pub fn modifier(self) -> Elysian<N, V> {
        Elysian::PreModifier(self)
    }
}

use PreModifier::*;

#[derive(Debug)]
pub enum PostModifier<N, V> {
    Isosurface {
        dist: Expr<N, V>,
        shape: ElysianBox<N, V>,
    },
    Manifold {
        shape: ElysianBox<N, V>,
    },
}

impl<N, V> PostModifier<N, V> {
    pub fn modifier(self) -> Elysian<N, V> {
        Elysian::PostModifier(self)
    }
}

use PostModifier::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum Elysian<N, V> {
    Field(Field<N, V>),
    PreModifier(PreModifier<N, V>),
    PostModifier(PostModifier<N, V>),
    Combine {
        combinator: Vec<Combinator<N, V>>,
        shapes: ElysianList<N, V>,
    },
    Alias(Box<dyn Expand<N, V>>),
}

impl<N, V> Elysian<N, V> {
    pub fn isosurface(self, dist: Expr<N, V>) -> Elysian<N, V> {
        Elysian::PostModifier(Isosurface {
            dist,
            shape: self.into(),
        })
    }

    pub fn manifold(self) -> Elysian<N, V> {
        Elysian::PostModifier(Manifold { shape: self.into() })
    }

    pub fn translate(self, delta: Expr<N, V>) -> Elysian<N, V> {
        Elysian::PreModifier(Translate {
            delta,
            shape: self.into(),
        })
    }

    pub fn elongate(self, dir: Expr<N, V>, infinite: bool) -> Self {
        Elysian::PreModifier(Elongate {
            dir,
            infinite,
            shape: self.into(),
        })
    }
}
pub trait IntoCombine<N, V> {
    fn combine<U>(self, combinator: U) -> Elysian<N, V>
    where
        U: IntoIterator<Item = Combinator<N, V>>;
}

impl<N, V, T> IntoCombine<N, V> for T
where
    T: IntoIterator<Item = Elysian<N, V>>,
{
    fn combine<U>(self, combinator: U) -> Elysian<N, V>
    where
        U: IntoIterator<Item = Combinator<N, V>>,
    {
        Elysian::Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}

pub trait IntoAlias<N, V> {
    fn alias(self) -> Elysian<N, V>;
}

impl<N, V, T> IntoAlias<N, V> for T
where
    T: 'static + Expand<N, V>,
{
    fn alias(self) -> Elysian<N, V> {
        Elysian::Alias(Box::new(self))
    }
}
