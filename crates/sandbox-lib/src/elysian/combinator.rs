use super::{attribute::Attribute, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Boolean {
    Union,
    Intersection,
    Subtraction,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Blend<N, V> {
    SmoothUnion { attr: Attribute, k: Expr<N, V> },
    SmoothIntersection { attr: Attribute, k: Expr<N, V> },
    SmoothSubtraction { attr: Attribute, k: Expr<N, V> },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Combinator<N, V> {
    Boolean(Boolean),
    Blend(Blend<N, V>),
}
