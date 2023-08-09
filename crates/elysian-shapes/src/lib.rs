use combine::{Displace, Sided, SidedProp};
use elysian_core::{
    ast::{combine::Combinator, expr::IntoExpr},
    ir::{
        ast::{COLOR, DISTANCE, GRADIENT_2D},
        module::IntoAsIR,
    },
};
use field::{Chebyshev, Point};
use modify::{
    BoundType, IntoBasisBound, IntoDistanceBound, IntoIsosurface, IntoMirror, IntoSet,
    IntoTranslate,
};

pub mod central_diff_gradient;
pub mod combine;
pub mod cross_section;
pub mod derive_bounding_error;
pub mod elongate_basis;
pub mod field;
pub mod modify;
pub mod raymarch;
pub mod scale;
pub mod rotate;
pub mod derive_support_vector;
pub mod voronoi;

pub fn corner() -> impl IntoAsIR {
    Combinator::build()
        .push(Sided::left())
        .push(Displace::new(DISTANCE))
        .push(SidedProp::new(GRADIENT_2D, false))
        .combine()
        .push(Point.basis_bound(BoundType::Lower, [0.0, 0.0]))
        .push(Chebyshev.distance_bound(BoundType::Upper, 0.0))
}

pub fn quad(extent: impl IntoExpr) -> impl IntoAsIR {
    corner().translate(extent).mirror_basis([1.0, 1.0])
}

pub fn local_origin() -> impl IntoAsIR {
    Point.isosurface(0.05).set_post(COLOR, [1.0, 0.0, 0.0, 1.0])
}
