use std::fmt::Debug;

use tracing::instrument;

use crate::ast::{Elysian, PostModifier, PreModifier};

pub trait Expand<N, V>: Debug {
    fn expand(&self) -> Elysian<N, V>;
}

impl<N, V> Expand<N, V> for Elysian<N, V>
where
    N: Debug + Copy,
    V: Debug + Copy,
{
    #[instrument]
    fn expand(&self) -> Elysian<N, V> {
        match self {
            Elysian::Field {
                pre_modifiers,
                field,
                post_modifiers,
            } => Elysian::Field {
                pre_modifiers: pre_modifiers
                    .iter()
                    .map(|modifier| match modifier {
                        PreModifier::Translate { delta } => PreModifier::Translate {
                            delta: delta.clone(),
                        },
                        PreModifier::Elongate { dir, infinite } => PreModifier::Elongate {
                            dir: dir.clone(),
                            infinite: *infinite,
                        },
                    })
                    .collect(),
                field: *field,
                post_modifiers: post_modifiers
                    .iter()
                    .map(|modifier| match modifier {
                        PostModifier::Isosurface { dist } => {
                            PostModifier::Isosurface { dist: dist.clone() }
                        }
                        PostModifier::Manifold => PostModifier::Manifold,
                    })
                    .collect(),
            },
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
