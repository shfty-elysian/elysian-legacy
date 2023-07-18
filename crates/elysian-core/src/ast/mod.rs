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

use crate::ir::{as_ir::AsIR, ast::{TypeSpec, VectorSpace}};

use self::{
    expr::Expr,
    post_modifier::{isosurface::Isosurface, manifold::Manifold},
    pre_modifier::{elongate::Elongate, translate::Translate},
};

pub type ElysianBox<T, const N: usize> = Box<Elysian<T, N>>;
pub type ElysianList<T, const N: usize> = Vec<Elysian<T, N>>;

pub mod post_modifier;
pub mod pre_modifier;

#[non_exhaustive]
pub enum Elysian<T, const N: usize> {
    Field {
        pre_modifiers: Vec<Box<dyn AsIR<T, N>>>,
        field: Box<dyn AsIR<T, N>>,
        post_modifiers: Vec<Box<dyn AsIR<T, N>>>,
    },
    Combine {
        combinator: Vec<Box<dyn AsIR<T, N>>>,
        shapes: ElysianList<T, N>,
    },
}

impl<T, const N: usize> Debug for Elysian<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => f
                .debug_struct("Field")
                .field("pre_modifiers", pre_modifiers)
                .field("field", field)
                .field("post_modifiers", post_modifiers)
                .finish(),
            Self::Combine { combinator, shapes } => f
                .debug_struct("Combine")
                .field("combinator", combinator)
                .field("shapes", shapes)
                .finish(),
        }
    }
}

impl<T, const N: usize> Hash for Elysian<T, N> {
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
                for combinator in combinator {
                    state.write_u64(combinator.hash_ir());
                }
                shapes.hash(state);
            }
        }
    }
}

impl<T, const N: usize> Elysian<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    pub fn shape_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn isosurface(self, dist: Expr<T>) -> Elysian<T, N> {
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

    pub fn manifold(self) -> Elysian<T, N> {
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

    pub fn translate(self, delta: Expr<T>) -> Elysian<T, N> {
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

    pub fn elongate(self, dir: Expr<T>, infinite: bool) -> Self {
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
pub trait IntoCombine<T, const N: usize> {
    fn combine<U>(self, combinator: U) -> Elysian<T, N>
    where
        U: IntoIterator<Item = Box<dyn AsIR<T, N>>>;
}

impl<T, U, const N: usize> IntoCombine<U, N> for T
where
    T: IntoIterator<Item = Elysian<U, N>>,
{
    fn combine<V>(self, combinator: V) -> Elysian<U, N>
    where
        V: IntoIterator<Item = Box<dyn AsIR<U, N>>>,
    {
        Elysian::Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}
