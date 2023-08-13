use elysian_core::{
    expr::{IntoLiteral, IntoPath},
    property_identifier::IntoPropertyIdentifier,
};
use elysian_ir::ast::{POSITION_2D, X, Y};
use elysian_shapes::{
    combine::{Combine, Union},
    elongate_basis::IntoElongateBasis,
    field::{Arc, Circle, Line, Point},
    filter::IntoFilter,
    mirror::IntoMirror,
    modify::{
        ClampMode, IntoElongateAxis, IntoFlipBasis, IntoManifold, IntoModify, IntoRepeat,
        IntoTranslate, REPEAT_ID_2D,
    },
    prepass::IntoPrepass,
    rotate::IntoRotate,
    select::Select,
    shape::IntoShape,
};

use super::combinator;

pub fn a(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([cell_size[0] * 0.75, 0.0]).translate([0.0, cell_size[1] * -0.5]))
        .push(Line::segment([cell_size[0], cell_size[1] * -2.0]).translate([0.0, cell_size[1]]))
        .mirror_basis([1.0, 0.0])
        .shape()
}

pub fn b(cell_size: [f64; 2]) -> impl IntoShape {
    let radius = cell_size[1] * 0.5;

    Combine::from(combinator())
        .push(Line::segment([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .push(
            Select::new(
                Circle::new(radius)
                    .manifold()
                    .translate([cell_size[0] - radius, cell_size[1] - radius]),
            )
            .case(
                POSITION_2D.path().push(X).read().lt(cell_size[0] - radius),
                Line::segment([cell_size[0] * 2.0, 0.0])
                    .translate([-cell_size[0], cell_size[1] * 0.5])
                    .mirror_basis([0.0, 1.0])
                    .translate([0.0, cell_size[1] * 0.5]),
            )
            .modify(),
        )
        .mirror_basis([0.0, 1.0])
}

pub fn c(cell_size: [f64; 2]) -> impl IntoShape {
    j(cell_size).flip_basis([1.0, 1.0]).mirror_basis([0.0, 1.0])
}

pub fn d(cell_size: [f64; 2]) -> impl IntoShape {
    let repeat_id = REPEAT_ID_2D.prop();

    let repeat_id_x_lt = |t: f64| repeat_id.clone().path().push(X).read().clone().lt(t);

    Select::new(o(cell_size))
        .case(
            repeat_id_x_lt(cell_size[0]),
            Combine::from(Union)
                .push(Line::segment([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
                .push(Line::segment([cell_size[0], 0.0]).translate([-cell_size[0], cell_size[1]]))
                .mirror_basis([0.0, 1.0]),
        )
        .modify()
        .prepass(
            Point
                .repeat_clamped(cell_size, [0.0, 0.0], cell_size)
                .translate([-cell_size[0] * 0.5, 0.0])
                .filter(REPEAT_ID_2D),
        )
}

pub fn e(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .push(Line::segment([cell_size[0], 0.0]).translate([-cell_size[0], 0.0]))
        .push(Line::centered([cell_size[0], 0.0]).translate([0.0, cell_size[1]]))
        .mirror_basis([0.0, 1.0])
}

pub fn f(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, cell_size[1]]).translate([-cell_size[0], -0.0]))
        .push(Line::segment([cell_size[0], 0.0]).translate([-cell_size[0], 0.0]))
        .push(Line::centered([cell_size[0], 0.0]).translate([0.0, cell_size[1]]))
}

pub fn g(cell_size: [f64; 2]) -> impl IntoShape {
    let radius = cell_size[0].min(cell_size[1]);
    let e = if cell_size[0] < cell_size[1] {
        [0.0, cell_size[0].max(cell_size[1]) - radius]
    } else {
        [cell_size[0].max(cell_size[1]) - radius, 0.0]
    };

    Combine::from(Union)
        .push(
            Arc::new(135.0_f64.to_radians().literal(), radius.literal())
                .rotate(-22.5_f64.to_radians())
                .elongate_basis([e[0], 0.0])
                .translate([0.0, e[1]]),
        )
        .push(
            Arc::new(180.0_f64.to_radians().literal(), radius.literal())
                .rotate(180.0_f64.to_radians())
                .elongate_basis([e[0], 0.0])
                .translate([0.0, -e[1]]),
        )
        .push(Line::segment([cell_size[0], 0.0]))
        .push(Line::centered([0.0, e[1]]).translate([-cell_size[0], 0.0]))
        .push(Line::segment([0.0, -e[1]]).translate([cell_size[0], 0.0]))
}

pub fn h(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, cell_size[1]]).translate([cell_size[0], 0.0]))
        .push(Line::segment([cell_size[0], 0.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn i(cell_size: [f64; 2]) -> impl IntoShape {
    Line::centered([0.0, cell_size[1]])
}

pub fn j(cell_size: [f64; 2]) -> impl IntoShape {
    Select::new(
        Combine::from(Union)
            .push(Line::segment([-cell_size[0], 0.0]).translate([0.0, -cell_size[1]]))
            .push(Line::segment([0.0, cell_size[1]]).translate([cell_size[0], 0.0])),
    )
    .case(
        POSITION_2D
            .path()
            .push(X)
            .read()
            .gt(0.0)
            .and(POSITION_2D.path().push(Y).read().lt(0.0)),
        o(cell_size),
    )
}

pub fn k(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .push(Line::segment([cell_size[0] * 2.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn l(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .push(Line::centered([cell_size[0], 0.0]).translate([0.0, -cell_size[1]]))
}

pub fn m(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, cell_size[1] * 2.0]).translate([cell_size[0], -cell_size[1]]))
        .push(Line::segment(cell_size))
        .mirror_basis([1.0, 0.0])
}

pub fn n(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(
            Line::segment([0.0, cell_size[1]])
                .translate([cell_size[0], 0.0])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([-cell_size[0], cell_size[1]]))
}

pub fn o(cell_size: [f64; 2]) -> impl IntoShape {
    let diff = cell_size[0].max(cell_size[1]) - cell_size[0].min(cell_size[1]);
    let (r, e) = if cell_size[0] > cell_size[1] {
        (cell_size[1], [diff, 0.0])
    } else {
        (cell_size[0], [0.0, diff])
    };
    Circle::new(r).elongate_basis(e).manifold()
}

pub fn p(cell_size: [f64; 2]) -> impl IntoShape {
    let radius = cell_size[1] * 0.5;

    Combine::from(combinator())
        .push(Line::centered([0.0, cell_size[1]]).translate([-cell_size[0], 0.0]))
        .push(
            Select::new(
                Circle::new(radius)
                    .manifold()
                    .translate([cell_size[0] - radius, cell_size[1] - radius]),
            )
            .case(
                POSITION_2D.path().push(X).read().lt(cell_size[0] - radius),
                Line::segment([cell_size[0] * 2.0, 0.0])
                    .translate([-cell_size[0], cell_size[1] * 0.5])
                    .mirror_basis([0.0, 1.0])
                    .translate([0.0, cell_size[1] * 0.5]),
            )
            .prepass(
                Point
                    .repeat_clamped(cell_size, [0.0, 0.0], [cell_size[0], 0.0])
                    .translate([cell_size[0] * -0.25, 0.0])
                    .filter(REPEAT_ID_2D),
            ),
        )
}

pub fn q(cell_size: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator()).push(o(cell_size)).push(
        Line::segment([cell_size[0] * 0.5, cell_size[1] * -0.5])
            .translate([cell_size[0] * 0.5, cell_size[1] * -0.5]),
    )
}

pub fn r(cell_size: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator())
        .push(p(cell_size))
        .push(Line::segment([cell_size[0] * 2.0, -cell_size[1]]).translate([-cell_size[0], 0.0]))
}

pub fn s(cell_size: [f64; 2]) -> impl IntoShape {
    let radius = cell_size[1] * 0.5;

    Select::new(
        Combine::from(Union)
            .push(Line::segment([radius, 0.0]).translate([-radius, -cell_size[1]]))
            .push(
                Circle::new(radius)
                    .manifold()
                    .translate([0.0, cell_size[1] - radius]),
            ),
    )
    .case(
        POSITION_2D.path().push(X).read().gt(0.0),
        Combine::from(Union)
            .push(Line::segment([-radius, 0.0]).translate([radius, cell_size[1]]))
            .push(
                Circle::new(radius)
                    .manifold()
                    .translate([0.0, -(cell_size[1] - radius)]),
            ),
    )
    .elongate_axis(
        [(cell_size[0] - 0.5).max(0.0), 0.0],
        ClampMode::Dir,
        ClampMode::Dir,
    )
}

pub fn t(cell_size: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator())
        .push(Line::centered([cell_size[0], 0.0]).translate([0.0, cell_size[1]]))
        .push(Line::centered([0.0, cell_size[1]]))
}

pub fn u(cell_size: [f64; 2]) -> impl IntoShape {
    j(cell_size).mirror_basis([1.0, 0.0])
}

pub fn v(cell_size: [f64; 2]) -> impl IntoShape {
    Line::segment([cell_size[0], cell_size[1] * 2.0])
        .translate([0.0, -cell_size[1]])
        .mirror_basis([1.0, 0.0])
}

pub fn w(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(
            Line::segment([cell_size[0] * 0.5, cell_size[1] * 2.0])
                .translate([cell_size[0] * 0.5, cell_size[1] * -1.0]),
        )
        .push(Line::segment([cell_size[0] * 0.5, cell_size[1] * -1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn x(cell_size: [f64; 2]) -> impl IntoShape {
    Line::segment(cell_size).mirror_basis([1.0, 1.0])
}

pub fn y(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment(cell_size))
        .push(Line::segment([0.0, -cell_size[1]]))
        .mirror_basis([1.0, 0.0])
}

pub fn z(cell_size: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(
            Line::segment([cell_size[0], 0.0])
                .translate([0.0, cell_size[1]])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered(cell_size))
}
