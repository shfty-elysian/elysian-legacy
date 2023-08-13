//! Built-in shapes and operators; the bulk of Elysian's user-facing API.

pub mod central_diff_gradient;
pub mod color;
pub mod combine;
pub mod derive_bounding_error;
pub mod field;
pub mod modify;
pub mod prepass;
pub mod raymarch;
pub mod select;
pub mod shape;
pub mod voronoi;
pub mod wrap;

use elysian_ir::ast::COLOR;
use field::Point;
use modify::{IntoIsosurface, IntoSet};
use shape::IntoShape;

pub fn local_origin() -> impl IntoShape {
    Point.isosurface(0.05).set_post(COLOR, [1.0, 0.0, 0.0, 1.0])
}
