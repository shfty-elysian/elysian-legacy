use elysian_core::{
    expr::{IntoLiteral, IntoPath},
    property_identifier::IntoPropertyIdentifier,
};
use elysian_ir::ast::{POSITION_2D, X, Y};
use elysian_shapes::{
    combine::{Combine, Union},
    field::{Arc, Circle, Line, Point},
    modify::{
        ClampMode, IntoElongateAxis, IntoFlipBasis, IntoManifold, IntoModify, IntoRepeat,
        IntoTranslate, REPEAT_ID_2D,
    },
    prepass::IntoPrepass,
    select::Select,
    shape::IntoShape,
    wrap::{
        elongate_basis::IntoElongateBasis, filter::IntoFilter, mirror::IntoMirror,
        rotate::IntoRotate,
    },
};

use super::combinator;

pub fn a([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([width * 0.75, 0.0]).translate([0.0, height * -0.5]))
        .push(Line::segment([width, height * -2.0]).translate([0.0, height]))
        .mirror_basis([1.0, 0.0])
        .shape()
}

pub fn b([width, height]: [f64; 2]) -> impl IntoShape {
    let radius = height * 0.5;

    Combine::from(combinator())
        .push(Line::segment([0.0, height]).translate([-width, 0.0]))
        .push(
            Select::new(
                Circle::new(radius)
                    .manifold()
                    .translate([width - radius, height - radius]),
            )
            .case(
                POSITION_2D.path().push(X).read().lt(width - radius),
                Line::segment([width * 2.0, 0.0])
                    .translate([-width, height * 0.5])
                    .mirror_basis([0.0, 1.0])
                    .translate([0.0, height * 0.5]),
            )
            .modify(),
        )
        .mirror_basis([0.0, 1.0])
}

pub fn c(cell_size: [f64; 2]) -> impl IntoShape {
    j(cell_size).flip_basis([1.0, 1.0]).mirror_basis([0.0, 1.0])
}

pub fn d(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    let repeat_id = REPEAT_ID_2D.prop();

    let repeat_id_x_lt = |t: f64| repeat_id.clone().path().push(X).read().clone().lt(t);

    Select::new(o(cell_size))
        .case(
            repeat_id_x_lt(width),
            Combine::from(Union)
                .push(Line::segment([0.0, height]).translate([-width, 0.0]))
                .push(Line::segment([width, 0.0]).translate([-width, height]))
                .mirror_basis([0.0, 1.0]),
        )
        .modify()
        .prepass(
            Point
                .repeat_clamped(cell_size, [0.0, 0.0], cell_size)
                .translate([-width * 0.5, 0.0])
                .filter(REPEAT_ID_2D),
        )
}

pub fn e([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, height]).translate([-width, 0.0]))
        .push(Line::segment([width, 0.0]).translate([-width, 0.0]))
        .push(Line::centered([width, 0.0]).translate([0.0, height]))
        .mirror_basis([0.0, 1.0])
}

pub fn f([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, height]).translate([-width, -0.0]))
        .push(Line::segment([width, 0.0]).translate([-width, 0.0]))
        .push(Line::centered([width, 0.0]).translate([0.0, height]))
}

pub fn g([width, height]: [f64; 2]) -> impl IntoShape {
    let radius = width.min(height);
    let [ex, ey] = if width < height {
        [0.0, width.max(height) - radius]
    } else {
        [width.max(height) - radius, 0.0]
    };

    Combine::from(Union)
        .push(
            Arc::new(135.0_f64.to_radians().literal(), radius.literal())
                .rotate(-22.5_f64.to_radians())
                .elongate_basis([ex, 0.0])
                .translate([0.0, ey]),
        )
        .push(
            Arc::new(180.0_f64.to_radians().literal(), radius.literal())
                .rotate(180.0_f64.to_radians())
                .elongate_basis([ex, 0.0])
                .translate([0.0, -ey]),
        )
        .push(Line::segment([width, 0.0]))
        .push(Line::centered([0.0, ex]).translate([-width, 0.0]))
        .push(Line::segment([0.0, -ey]).translate([width, 0.0]))
}

pub fn h([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, height]).translate([width, 0.0]))
        .push(Line::segment([width, 0.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn i([_, height]: [f64; 2]) -> impl IntoShape {
    Line::centered([0.0, height])
}

pub fn j(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    Select::new(
        Combine::from(Union)
            .push(Line::segment([-width, 0.0]).translate([0.0, -height]))
            .push(Line::segment([0.0, height]).translate([width, 0.0])),
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

pub fn k([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, height]).translate([-width, 0.0]))
        .push(Line::segment([width * 2.0, height]).translate([-width, 0.0]))
        .mirror_basis([0.0, 1.0])
}

pub fn l([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::centered([0.0, height]).translate([-width, 0.0]))
        .push(Line::centered([width, 0.0]).translate([0.0, -height]))
}

pub fn m(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([0.0, height * 2.0]).translate([width, -height]))
        .push(Line::segment(cell_size))
        .mirror_basis([1.0, 0.0])
}

pub fn n([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(
            Line::segment([0.0, height])
                .translate([width, 0.0])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered([-width, height]))
}

pub fn o([width, height]: [f64; 2]) -> impl IntoShape {
    let diff = width.max(height) - width.min(height);
    let (r, e) = if width > height {
        (height, [diff, 0.0])
    } else {
        (width, [0.0, diff])
    };
    Circle::new(r).elongate_basis(e).manifold()
}

pub fn p(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    let radius = height * 0.5;

    Combine::from(combinator())
        .push(Line::centered([0.0, height]).translate([-width, 0.0]))
        .push(
            Select::new(
                Circle::new(radius)
                    .manifold()
                    .translate([width - radius, height - radius]),
            )
            .case(
                POSITION_2D.path().push(X).read().lt(width - radius),
                Line::segment([width * 2.0, 0.0])
                    .translate([-width, height * 0.5])
                    .mirror_basis([0.0, 1.0])
                    .translate([0.0, height * 0.5]),
            )
            .prepass(
                Point
                    .repeat_clamped(cell_size, [0.0, 0.0], [width, 0.0])
                    .translate([width * -0.25, 0.0])
                    .filter(REPEAT_ID_2D),
            ),
        )
}

pub fn q(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator())
        .push(o(cell_size))
        .push(Line::segment([width * 0.5, height * -0.5]).translate([width * 0.5, height * -0.5]))
}

pub fn r(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator())
        .push(p(cell_size))
        .push(Line::segment([width * 2.0, -height]).translate([-width, 0.0]))
}

pub fn s([width, height]: [f64; 2]) -> impl IntoShape {
    let radius = height * 0.5;

    Select::new(
        Combine::from(Union)
            .push(Line::segment([radius, 0.0]).translate([-radius, -height]))
            .push(
                Circle::new(radius)
                    .manifold()
                    .translate([0.0, height - radius]),
            ),
    )
    .case(
        POSITION_2D.path().push(X).read().gt(0.0),
        Combine::from(Union)
            .push(Line::segment([-radius, 0.0]).translate([radius, height]))
            .push(
                Circle::new(radius)
                    .manifold()
                    .translate([0.0, -(height - radius)]),
            ),
    )
    .elongate_axis(
        [(width - 0.5).max(0.0), 0.0],
        ClampMode::Dir,
        ClampMode::Dir,
    )
}

pub fn t([width, height]: [f64; 2]) -> impl IntoShape {
    Combine::from(combinator())
        .push(Line::centered([width, 0.0]).translate([0.0, height]))
        .push(Line::centered([0.0, height]))
}

pub fn u(cell_size: [f64; 2]) -> impl IntoShape {
    j(cell_size).mirror_basis([1.0, 0.0])
}

pub fn v([width, height]: [f64; 2]) -> impl IntoShape {
    Line::segment([width, height * 2.0])
        .translate([0.0, -height])
        .mirror_basis([1.0, 0.0])
}

pub fn w([width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment([width * 0.5, height * 2.0]).translate([width * 0.5, height * -1.0]))
        .push(Line::segment([width * 0.5, height * -1.0]))
        .mirror_basis([1.0, 0.0])
}

pub fn x(cell_size @ [_, _]: [f64; 2]) -> impl IntoShape {
    Line::segment(cell_size).mirror_basis([1.0, 1.0])
}

pub fn y(cell_size @ [_, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(Line::segment(cell_size))
        .push(Line::segment([0.0, -height]))
        .mirror_basis([1.0, 0.0])
}

pub fn z(cell_size @ [width, height]: [f64; 2]) -> impl IntoShape {
    combinator()
        .combine()
        .push(
            Line::segment([width, 0.0])
                .translate([0.0, height])
                .mirror_basis([1.0, 1.0]),
        )
        .push(Line::centered(cell_size))
}
