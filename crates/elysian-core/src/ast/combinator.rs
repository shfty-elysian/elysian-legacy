use super::{attribute::Attribute, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boolean {
    Union,
    Intersection,
    Subtraction,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blend<N, V> {
    SmoothUnion { attr: Attribute, k: Expr<N, V> },
    SmoothIntersection { attr: Attribute, k: Expr<N, V> },
    SmoothSubtraction { attr: Attribute, k: Expr<N, V> },
}

impl<N, V> std::hash::Hash for Blend<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Blend::SmoothUnion { attr, k } => {
                attr.hash(state);
                k.hash(state);
            }
            Blend::SmoothIntersection { attr, k } => {
                attr.hash(state);
                k.hash(state);
            }
            Blend::SmoothSubtraction { attr, k } => {
                attr.hash(state);
                k.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Combinator<N, V> {
    Boolean(Boolean),
    Blend(Blend<N, V>),
}

impl<N, V> std::hash::Hash for Combinator<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Combinator::Boolean(b) => b.hash(state),
            Combinator::Blend(b) => b.hash(state),
        }
    }
}
