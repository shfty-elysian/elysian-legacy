use super::{
    attribute::Attribute,
    value::{IntoValue, Value},
};

pub type BoxExpr<N, V> = Box<Expr<N, V>>;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr<N, V> {
    Literal(Value<N, V>),
    Read(Attribute),
    Add(BoxExpr<N, V>, BoxExpr<N, V>),
    Sub(BoxExpr<N, V>, BoxExpr<N, V>),
    Mul(BoxExpr<N, V>, BoxExpr<N, V>),
    Div(BoxExpr<N, V>, BoxExpr<N, V>),
    Min(BoxExpr<N, V>, BoxExpr<N, V>),
    Max(BoxExpr<N, V>, BoxExpr<N, V>),
    Mix(BoxExpr<N, V>, BoxExpr<N, V>, BoxExpr<N, V>),
    Lt(BoxExpr<N, V>, BoxExpr<N, V>),
    Gt(BoxExpr<N, V>, BoxExpr<N, V>),
    Neg(BoxExpr<N, V>),
    Abs(BoxExpr<N, V>),
    Sign(BoxExpr<N, V>),
    Length(BoxExpr<N, V>),
    Normalize(BoxExpr<N, V>),
    Dot(BoxExpr<N, V>, BoxExpr<N, V>),
}

impl<N, V> std::hash::Hash for Expr<N, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait IntoLiteral<N, V>: IntoValue<N, V> {
    fn literal(self) -> Expr<N, V>;
}

impl<T, N, V> IntoLiteral<N, V> for T
where
    T: IntoValue<N, V>,
{
    fn literal(self) -> Expr<N, V> {
        Expr::Literal(self.value())
    }
}
