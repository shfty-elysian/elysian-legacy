use elysian_core::{
    ast::identifier::Identifier,
    ir::module::{NumericType, Type},
    property,
};

pub const K: Identifier = Identifier::new("k", 12632115441234896764);
property!(K, K_PROP, Type::Number(NumericType::Float));

mod smooth_intersection;
mod smooth_subtraction;
mod smooth_union;

pub use smooth_intersection::*;
pub use smooth_subtraction::*;
pub use smooth_union::*;
