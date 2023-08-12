use elysian_shapes::{
    field::Line,
    modify::{IntoMirror, IntoTranslate},
    shape::IntoShape,
};

use super::combinator;

pub fn sigma() -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([2.0, 0.0]).translate([-1.0, 1.0]))
        .push(Line::segment([-0.8, 0.8]))
        .mirror_basis([0.0, 1.0])
}
