mod capsule;
mod circle;
mod line;
mod point;
mod ring;

pub use capsule::*;
pub use circle::*;
pub use line::*;
pub use point::*;
pub use ring::*;

use crate::ir::{as_ir::AsIR, ast::{TypeSpec, VectorSpace}};

use super::Elysian;

pub trait IntoField<T, const N: usize>: 'static + Sized + AsIR<T, N>
where
    T: TypeSpec + VectorSpace<N>,
{
    fn field(self) -> Elysian<T, N> {
        Elysian::Field {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, U, const N: usize> IntoField<U, N> for T
where
    T: 'static + Sized + AsIR<U, N>,
    U: TypeSpec + VectorSpace<N>,
{
}
