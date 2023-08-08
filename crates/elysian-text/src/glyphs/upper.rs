use elysian_core::{
    ast::{
        combine::Combine, expr::IntoLiteral, filter::IntoFilter, modify::IntoModify, select::Select,
    },
    ir::{
        ast::{POSITION_2D, X, Y},
        module::IntoAsIR,
    },
};
use elysian_shapes::{
    combine::Union,
    corner,
    elongate_basis::IntoElongateBasis,
    field::{Circle, Infinity, Line, Point},
    modify::{
        IntoFlipBasis, IntoIsosurface, IntoManifold, IntoMirror, IntoRepeat, IntoTranslate,
        REPEAT_ID_2D,
    },
};

use super::combinator;

pub fn a() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.66, 0.0]).translate([0.0, -0.5]))
        .push(Line::segment([1.0, -2.25]).translate([0.0, 1.0]))
        .mirror_basis([1.0, 0.0])
        .as_ir()
}

pub fn b() -> impl IntoAsIR {
    Combine::from(combinator())
        .push(Line::segment([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(
            Select::new(Circle::new(0.5).manifold().translate([0.25, 0.5]))
                .case(
                    REPEAT_ID_2D.path().push(X).read().lt(1.0),
                    Line::segment([1.25, 0.0])
                        .translate([-1.0, 0.5])
                        .mirror_basis([0.0, 1.0])
                        .translate([0.0, 0.5]),
                )
                .modify()
                .push_pre(
                    Point
                        .repeat_clamped([1.0, 1.0], [0.0, 0.0], [1.0, 0.0])
                        .translate([-0.25, 0.0])
                        .filter(REPEAT_ID_2D),
                ),
        )
        .mirror_basis([0.0, 1.0])
}

pub fn c() -> impl IntoAsIR {
    Select::new(Line::segment([0.5, 0.0]).translate([0.0, 1.0]))
        .case(
            REPEAT_ID_2D.path().push(X).read().lt(1.0),
            Circle::new(1.0).manifold().translate([-0.25, 0.0]),
        )
        .modify()
        .push_pre(
            Point
                .repeat_clamped([1.0, 1.0], [0.0, 0.0], [1.0, 0.0])
                .translate([-0.5, 0.0])
                .filter(REPEAT_ID_2D),
        )
        .mirror_basis([0.0, 1.0])
}

pub fn d() -> impl IntoAsIR {
    let repeat_id = REPEAT_ID_2D.prop();

    let repeat_id_x_lt = |t: f64| repeat_id.clone().path().push(X).read().clone().lt(t);

    Select::new(Circle::new(1.0).manifold().translate([0.0, 0.0]))
        .case(
            repeat_id_x_lt(1.0),
            Line::segment([0.0, 1.0])
                .translate([-1.0, 0.0])
                .mirror_axis([-1.0, -1.0].literal().normalize())
                .mirror_basis([0.0, 1.0]),
        )
        .modify()
        .push_pre(
            Point
                .repeat_clamped([1.0, 1.0], [0.0, 0.0], [1.0, 1.0])
                .translate([-0.5, 0.0])
                .filter(REPEAT_ID_2D),
        )
}

pub fn e() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(Line::segment([1.0, 0.0]).translate([-1.0, 0.0]))
        .push(Line::centered([1.0, 0.0]).translate([0.0, 1.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn f() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([-1.0, -0.0]))
        .push(Line::segment([1.0, 0.0]).translate([-1.0, 0.0]))
        .push(Line::centered([1.0, 0.0]).translate([0.0, 1.0]))
}

pub fn g() -> impl IntoAsIR {
    let repeat_id = REPEAT_ID_2D.prop();
    let repeat_id_x_gt = |t: f64| repeat_id.clone().path().push(X).read().clone().gt(t);
    let repeat_id_y_gt = |t: f64| repeat_id.clone().path().push(Y).read().clone().gt(t);
    let repeat_id_y_lt = |t: f64| repeat_id.clone().path().push(Y).read().clone().lt(t);

    Select::new(Circle::new(1.0).elongate_basis([0.125, 0.25]).manifold())
        .case(
            repeat_id_x_gt(-0.25).and(repeat_id_y_gt(0.0)),
            Line::centered([1.0, 0.0]).translate([0.0, 1.25]),
        )
        .case(
            repeat_id_x_gt(-0.25)
                .and(repeat_id_y_gt(-0.75))
                .and(repeat_id_y_lt(0.75)),
            Combine::from(combinator())
                .push(Line::segment([1.1, 0.0]))
                .push(Line::segment([0.0, -1.0]).translate([1.1, 0.0])),
        )
        .modify()
        .push_pre(
            Point
                .repeat_clamped([0.25, 0.75], [-0.25, -0.75], [0.25, 0.75])
                .translate([-0.25, 0.0])
                .filter(REPEAT_ID_2D),
        )
}

pub fn h() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([1.0, 0.0]))
        .push(Line::segment([1.0, 0.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn i() -> impl IntoAsIR {
    Line::centered([0.0, 1.0])
}

pub fn j() -> impl IntoAsIR {
    Select::new(
        Combine::from(Union)
            .push(Line::segment([-0.5, 0.0]).translate([0.0, -1.0]))
            .push(Line::segment([0.0, 1.25]).translate([1.0, 0.0])),
    )
    .case(
        POSITION_2D
            .path()
            .push(X)
            .read()
            .gt(0.0)
            .and(POSITION_2D.path().push(Y).read().lt(0.0)),
        corner().flip_basis([0.0, 1.0]).isosurface(1.0).manifold(),
    )
    .translate([-0.25, -0.25])
}

pub fn k() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 1.0]).translate([-0.7, 0.0]))
        .push(Line::segment([1.0, 1.0]).translate([-0.5, 0.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn l() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(Line::centered([1.0, 0.0]).translate([0.0, -1.0]))
}

pub fn m() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.0, 2.0]).translate([1.0, -1.0]))
        .push(Line::segment([1.0, 1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn n() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(
            Line::segment([0.0, 1.0])
                .translate([1.0, 0.0])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([-1.0, 1.0]))
}

pub fn o() -> impl IntoAsIR {
    Circle::new(1.0).elongate_basis([0.125, 0.25]).manifold()
}

pub fn p() -> impl IntoAsIR {
    Combine::from(combinator())
        .push(Line::centered([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(
            Select::new(Circle::new(0.5).manifold().translate([0.25, 0.5]))
                .case(
                    REPEAT_ID_2D.path().push(X).read().lt(1.0),
                    Line::segment([1.25, 0.0])
                        .translate([-1.0, 0.5])
                        .mirror_basis([0.0, 1.0])
                        .translate([0.0, 0.5]),
                )
                .modify()
                .push_pre(
                    Point
                        .repeat_clamped([1.0, 1.0], [0.0, 0.0], [1.0, 0.0])
                        .translate([-0.25, 0.0])
                        .filter(REPEAT_ID_2D),
                ),
        )
}

pub fn q() -> impl IntoAsIR {
    Combine::from(combinator())
        .push(Circle::new(1.0).elongate_basis([0.125, 0.25]).manifold())
        .push(Line::segment([0.75, -0.75]).translate([0.5, -0.5]))
}

pub fn r() -> impl IntoAsIR {
    Combine::from(combinator())
        .push(Line::centered([0.0, 1.0]).translate([-1.0, 0.0]))
        .push(Line::segment([1.6, -1.0]).translate([-1.0, 0.0]))
        .push(
            Select::new(Circle::new(0.5).manifold().translate([0.25, 0.5]))
                .case(
                    REPEAT_ID_2D.path().push(X).read().lt(1.0),
                    Line::segment([1.25, 0.0])
                        .translate([-1.0, 0.5])
                        .mirror_basis([0.0, 1.0])
                        .translate([0.0, 0.5]),
                )
                .modify()
                .push_pre(
                    Point
                        .repeat_clamped([1.0, 1.0], [0.0, 0.0], [1.0, 0.0])
                        .translate([-0.25, 0.0])
                        .filter(REPEAT_ID_2D),
                ),
        )
}

pub fn s() -> impl IntoAsIR {
    let repeat_id = REPEAT_ID_2D.prop();

    let repeat_id_x_lt = |t: f64| repeat_id.clone().path().push(X).read().clone().lt(t);
    let repeat_id_y_lt = |t: f64| repeat_id.clone().path().push(Y).read().clone().lt(t);

    let repeat_id_x_gt = |t: f64| repeat_id.clone().path().push(X).read().clone().gt(t);
    let repeat_id_y_gt = |t: f64| repeat_id.clone().path().push(Y).read().clone().gt(t);

    Select::new(Line::centered([1.0, 0.0]))
        .case(
            repeat_id_x_lt(1.0).and(repeat_id_y_lt(0.0)),
            Line::segment([1.5, 0.0]).translate([-1.0, -1.0]),
        )
        .case(
            repeat_id_x_gt(0.0).and(repeat_id_y_lt(1.0)),
            Circle::new(0.5).manifold().translate([0.5, -0.5]),
        )
        .case(
            repeat_id_x_lt(0.0).and(repeat_id_y_gt(-1.0)),
            Circle::new(0.5).manifold().translate([-0.5, 0.5]),
        )
        .case(
            repeat_id_x_gt(-1.0).and(repeat_id_y_gt(0.0)),
            Line::segment([-1.5, 0.0]).translate([1.0, 1.0]),
        )
        .modify()
        .push_pre(
            Point
                .repeat_clamped([1.0, 1.0], [-1.0, -1.0], [1.0, 1.0])
                .filter(REPEAT_ID_2D),
        )
}

pub fn t() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::centered([1.0, 0.0]))
        .push(Line::centered([0.0, 1.0]).translate([0.0, -1.0]))
        .translate([0.0, 1.0])
}

pub fn u() -> impl IntoAsIR {
    let repeat_id = REPEAT_ID_2D.prop();
    let repeat_id_y_gt = |t: f64| repeat_id.clone().path().push(Y).read().clone().gt(t);

    Select::new(Circle::new(1.0).elongate_basis([0.125, 0.25]).manifold())
        .case(
            repeat_id_y_gt(0.0),
            Line::centered([0.0, 1.0])
                .translate([1.125, 0.0])
                .mirror_basis([1.0, 0.0]),
        )
        .modify()
        .push_pre(
            Point
                .repeat_clamped([1.0, 1.0], [0.0, 0.0], [0.0, 1.0])
                .translate([0.0, -0.5])
                .filter(REPEAT_ID_2D),
        )
}

pub fn v() -> impl IntoAsIR {
    Line::segment([1.0, 2.0])
        .translate([0.0, -1.0])
        .mirror_basis([1.0, 0.0])
}

pub fn w() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([0.5, 2.0]).translate([0.5, -1.0]))
        .push(Line::segment([0.5, -1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn x() -> impl IntoAsIR {
    Line::segment([0.6, 1.0]).mirror_basis([1.0, 1.0])
}

pub fn y() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(Line::segment([1.0, 1.0]))
        .push(Line::segment([0.0, -1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn z() -> impl IntoAsIR {
    combinator()
        .combine()
        .push(
            Line::segment([0.7, 0.0])
                .translate([0.0, 1.0])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([0.7, 1.0]))
}
