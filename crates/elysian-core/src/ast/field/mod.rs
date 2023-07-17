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

use crate::ir::as_ir::AsIR;

use super::Elysian;

pub trait IntoField<N, V>: 'static + Sized + AsIR<N, V> {
    fn field(self) -> Elysian<N, V> {
        Elysian::Field {
            pre_modifiers: Default::default(),
            field: Box::new(self),
            post_modifiers: Default::default(),
        }
    }
}

impl<T, N, V> IntoField<N, V> for T where T: 'static + Sized + AsIR<N, V> {}
