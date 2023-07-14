use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type<N, V> {
    Boolean,
    Number,
    Vector,
    Struct(&'static str),
    _Phantom(PhantomData<(N, V)>),
}

