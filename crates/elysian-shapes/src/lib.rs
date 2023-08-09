use combine::{Displace, Sided, SidedProp};
use elysian_core::{
    ast::{combine::Combinator, expr::IntoExpr},
    ir::{
        ast::{COLOR, DISTANCE},
        module::{IntoAsIR, PropertyIdentifier},
    },
};
use field::{Chebyshev, Point};
use modify::{
    BoundType, IntoBasisBound, IntoDistanceBound, IntoIsosurface, IntoMirror, IntoSet,
    IntoTranslate,
};

pub mod central_diff_gradient;
pub mod color;
pub mod combine;
pub mod cross_section;
pub mod derive_bounding_error;
pub mod derive_support_vector;
pub mod elongate_basis;
pub mod field;
pub mod modify;
pub mod raymarch;
pub mod rotate;
pub mod scale;
pub mod voronoi;
pub mod uv_map;

pub fn corner(props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>) -> impl IntoAsIR {
    let combinator = Combinator::build()
        .push(Sided::left())
        .push(Displace::new(DISTANCE));

    props
        .into_iter()
        .map(Into::into)
        .fold(combinator, |acc, next| {
            acc.push(SidedProp::new(next, false))
        })
        .combine()
        .push(Point.basis_bound(BoundType::Lower, [0.0, 0.0]))
        .push(Chebyshev.distance_bound(BoundType::Upper, 0.0))
}

pub fn quad(
    extent: impl IntoExpr,
    props: impl IntoIterator<Item = impl Into<PropertyIdentifier>>,
) -> impl IntoAsIR {
    corner(props).translate(extent).mirror_basis([1.0, 1.0])
}

pub fn local_origin() -> impl IntoAsIR {
    Point.isosurface(0.05).set_post(COLOR, [1.0, 0.0, 0.0, 1.0])
}
