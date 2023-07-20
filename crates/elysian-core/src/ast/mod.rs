pub mod attribute;
pub mod central_diff_gradient;
pub mod combine;
pub mod cross_section;
pub mod expr;
pub mod field;
pub mod modify;
pub mod raymarch;
pub mod value;

use crate::ir::{as_ir::DynAsIR, module::DynAsModule};

use self::combine::Combine;

pub trait IntoCombine {
    fn combine<U>(self, combinator: U) -> Combine
    where
        U: IntoIterator<Item = DynAsIR>;
}

impl<T> IntoCombine for T
where
    T: IntoIterator<Item = DynAsModule>,
{
    fn combine<V>(self, combinator: V) -> Combine
    where
        V: IntoIterator<Item = DynAsIR>,
    {
        Combine {
            combinator: combinator.into_iter().collect(),
            shapes: self.into_iter().collect(),
        }
    }
}
