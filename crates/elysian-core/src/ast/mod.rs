pub mod alias;
pub mod attribute;
pub mod combinator;
pub mod expand;
pub mod expr;
pub mod field;
pub mod to_glam;
pub mod value;

use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use self::{
    combinator::Combinator,
    expand::Expand,
    expr::Expr,
    field::{AsField, Field},
};

pub type ElysianBox<N, V> = Box<Elysian<N, V>>;
pub type ElysianList<N, V> = Vec<Elysian<N, V>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreModifier<N, V> {
    Translate { delta: Expr<N, V> },
    Elongate { dir: Expr<N, V>, infinite: bool },
}

impl<N, V> Hash for PreModifier<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Translate { delta } => delta.hash(state),
            Elongate { dir, infinite } => {
                dir.hash(state);
                infinite.hash(state);
            }
        }
    }
}

use PreModifier::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostModifier<N, V> {
    Isosurface { dist: Expr<N, V> },
    Manifold,
}

impl<N, V> Hash for PostModifier<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Isosurface { dist } => dist.hash(state),
            Manifold => {}
        }
    }
}

use PostModifier::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum Elysian<N, V> {
    Field {
        pre_modifiers: Vec<PreModifier<N, V>>,
        field: Box<dyn AsField<N, V>>,
        post_modifiers: Vec<PostModifier<N, V>>,
    },
    Combine {
        combinator: Vec<Combinator<N, V>>,
        shapes: ElysianList<N, V>,
    },
    Alias(Box<dyn Expand<N, V>>),
}

/*
impl<N, V> Clone for Elysian<N, V>
where
    N: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Self::Field {
                pre_modifiers: pre_modifiers.clone(),
                field: field.clone(),
                post_modifiers: post_modifiers.clone(),
            },
            Self::Combine { combinator, shapes } => Self::Combine {
                combinator: combinator.clone(),
                shapes: shapes.clone(),
            },
            Self::Alias(_) => unimplemented!(),
        }
    }
}
*/

impl<N, V> Hash for Elysian<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => {
                pre_modifiers.hash(state);
                state.write_u64(field.field_hash());
                post_modifiers.hash(state);
            }
            Elysian::Combine { combinator, shapes } => {
                combinator.hash(state);
                shapes.hash(state);
            }
            Elysian::Alias(_) => {}
        }
    }
}

impl<N, V> Elysian<N, V>
where
    N: Debug,
    V: Debug,
{
    pub fn shape_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn isosurface(self, dist: Expr<N, V>) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers: post_modifiers
                    .into_iter()
                    .chain(std::iter::once(Isosurface { dist }))
                    .collect(),
            },
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn manifold(self) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers: post_modifiers
                    .into_iter()
                    .chain(std::iter::once(Manifold))
                    .collect(),
            },
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn translate(self, delta: Expr<N, V>) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Elysian::Field {
                pre_modifiers: pre_modifiers
                    .into_iter()
                    .chain(std::iter::once(Translate { delta }))
                    .collect(),
                field,
                post_modifiers,
            },
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn elongate(self, dir: Expr<N, V>, infinite: bool) -> Self {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Elysian::Field {
                pre_modifiers: pre_modifiers
                    .into_iter()
                    .chain(std::iter::once(Elongate { dir, infinite }))
                    .collect(),
                field,
                post_modifiers,
            },
            t => unimplemented!("{t:#?}"),
        }
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
