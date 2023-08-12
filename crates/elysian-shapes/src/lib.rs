//! Built-in shapes and operators

use elysian_ir::{ast::COLOR, module::IntoAsIR};
use field::Point;
use modify::{IntoIsosurface, IntoSet};

pub mod central_diff_gradient;
pub mod color;
pub mod combine;
pub mod cross_section;
pub mod derive_bounding_error;
pub mod derive_support_vector;
pub mod elongate_basis;
pub mod field;
pub mod filter;
pub mod modify;
pub mod raymarch;
pub mod rotate;
pub mod scale;
pub mod select;
pub mod uv_map;
pub mod voronoi;

pub fn local_origin() -> impl IntoAsIR {
    Point.isosurface(0.05).set_post(COLOR, [1.0, 0.0, 0.0, 1.0])
}
