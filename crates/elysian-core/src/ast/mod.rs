pub mod attribute;
pub mod combinator;
pub mod expr;
pub mod field;
pub mod value;

use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::ir::as_ir::AsIR;

use self::{
    combinator::Combinator,
    expr::Expr,
    post_modifier::{isosurface::Isosurface, manifold::Manifold},
    pre_modifier::{elongate::Elongate, translate::Translate},
};

pub type ElysianBox<N, V> = Box<Elysian<N, V>>;
pub type ElysianList<N, V> = Vec<Elysian<N, V>>;

pub mod post_modifier;
pub mod pre_modifier;

#[derive(Debug)]
#[non_exhaustive]
pub enum Elysian<N, V> {
    Field {
        pre_modifiers: Vec<Box<dyn AsIR<N, V>>>,
        field: Box<dyn AsIR<N, V>>,
        post_modifiers: Vec<Box<dyn AsIR<N, V>>>,
    },
    Combine {
        combinator: Vec<Combinator<N, V>>,
        shapes: ElysianList<N, V>,
    },
}

impl<N, V> Hash for Elysian<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => {
                for pre_modifier in pre_modifiers {
                    state.write_u64(pre_modifier.hash_ir());
                }
                state.write_u64(field.hash_ir());
                for post_modifier in post_modifiers {
                    state.write_u64(post_modifier.hash_ir());
                }
            }
            Elysian::Combine { combinator, shapes } => {
                combinator.hash(state);
                shapes.hash(state);
            }
        }
    }
}

impl<N, V> Elysian<N, V>
where
    N: 'static + Debug + Clone,
    V: 'static + Debug + Clone,
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
                mut post_modifiers,
            } => {
                post_modifiers.push(Box::new(Isosurface { dist }));
                Elysian::Field {
                    pre_modifiers,
                    field,
                    post_modifiers,
                }
            }
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn manifold(self) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                mut post_modifiers,
            } => {
                post_modifiers.push(Box::new(Manifold));
                Elysian::Field {
                    pre_modifiers,
                    field,
                    post_modifiers,
                }
            }
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn translate(self, delta: Expr<N, V>) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                mut pre_modifiers,
                field,
                post_modifiers,
            } => {
                pre_modifiers.push(Box::new(Translate { delta }));
                Elysian::Field {
                    pre_modifiers,
                    field,
                    post_modifiers,
                }
            }
            t => unimplemented!("{t:#?}"),
        }
    }

    pub fn elongate(self, dir: Expr<N, V>, infinite: bool) -> Self {
        match self {
            Elysian::Field {
                mut pre_modifiers,
                field,
                post_modifiers,
            } => {
                pre_modifiers.push(Box::new(Elongate { dir, infinite }));
                Elysian::Field {
                    pre_modifiers,
                    field,
                    post_modifiers,
                }
            }
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

