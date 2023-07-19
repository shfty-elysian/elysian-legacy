pub mod attribute;
pub mod combine;
pub mod expr;
pub mod field;
pub mod modify;
pub mod value;
pub mod central_diff_gradient;

use crate::ir::{as_ir::DynAsIR, module::DynAsModule};

use self::combine::Combine;

pub trait IntoCombine<T, const N: usize> {
    fn combine<U>(self, combinator: U) -> Combine<T, N>
    where
        U: IntoIterator<Item = DynAsIR<T, N>>;
}

impl<T, U, const N: usize> IntoCombine<U, N> for T
where
    T: IntoIterator<Item = DynAsModule<U, N>>,
{
    fn combine<V>(self, combinator: V) -> Combine<U, N>
    where
        V: IntoIterator<Item = DynAsIR<U, N>>,
    {
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}
