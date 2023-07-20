pub mod attribute;
pub mod central_diff_gradient;
pub mod combine;
pub mod cross_section;
pub mod expr;
pub mod field;
pub mod modify;
pub mod value;
pub mod raymarch;

use crate::ir::{as_ir::DynAsIR, module::DynAsModule};

use self::combine::Combine;

pub trait IntoCombine<T> {
    fn combine<U>(self, combinator: U) -> Combine<T>
    where
        U: IntoIterator<Item = DynAsIR<T>>;
}

impl<T, U> IntoCombine<U> for T
where
    T: IntoIterator<Item = DynAsModule<U>>,
{
    fn combine<V>(self, combinator: V) -> Combine<U>
    where
        V: IntoIterator<Item = DynAsIR<U>>,
    {
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}
