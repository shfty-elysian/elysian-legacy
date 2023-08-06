use elysian_core::{
    ast::combine::Combinator,
    ir::ast::{DISTANCE, GRADIENT_2D, UV},
};
use elysian_shapes::combine::{SmoothUnion, Union};

pub mod greek;
pub mod lower;
pub mod punct;
pub mod upper;

pub fn combinator() -> Combinator {
    Combinator::build()
        .push(Union)
        .push(SmoothUnion::new(DISTANCE, 0.4))
        .push(SmoothUnion::new(GRADIENT_2D, 0.4))
        .push(SmoothUnion::new(UV, 0.4))
}
