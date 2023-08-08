use elysian_core::{
    ast::{
        combine::{Combinator, Combine},
        expr::{Expr, IntoExpr, IntoLiteral, IntoRead},
        modify::IntoModify,
        select::Select,
    },
    ir::{
        ast::{COLOR, DISTANCE, GRADIENT_2D, NORMAL, POSITION_2D, UV, X, Y, Z},
        module::{DynAsIR, IntoAsIR},
    },
};
use elysian_shapes::{
    combine::{Displace, Sided, SidedProp, SmoothSubtraction, SmoothUnion, Subtraction, Union},
    field::{Arc, Capsule, Chebyshev, Circle, Infinity, Line, Point, Ring},
    modify::{
        BoundType, ClampMode, IntoAspect, IntoBasisBound, IntoCartesianToPolar, IntoDistanceBound,
        IntoElongateAxis, IntoFlipBasis, IntoGradientNormals, IntoIsosurface, IntoManifold,
        IntoMirror, IntoRepeat, IntoSet, IntoTranslate, ASPECT, REPEAT_ID_2D,
    },
    raymarch::Raymarch,
    scale::IntoScale,
    voronoi::{voronoi, CELL_ID},
};
use elysian_text::glyphs::{greek::sigma, text, Align};
use rust_gpu_bridge::glam::Mat4;

pub fn point() -> impl IntoAsIR {
    Point
        .gradient_normals()
        .set_post(COLOR, distance_normal_color())
}

pub fn chebyshev() -> impl IntoAsIR {
    Chebyshev
        .gradient_normals()
        .set_post(COLOR, distance_normal_color())
}

pub fn line() -> impl IntoAsIR {
    Line::centered([1.0, 0.0]).set_post(COLOR, uv_color())
}

pub fn circle() -> impl IntoAsIR {
    Circle::new(0.5)
        .gradient_normals()
        .set_post(COLOR, uv_color())
}

pub fn capsule() -> impl IntoAsIR {
    Capsule::new([1.5, 0.0], 0.5).set_post(COLOR, uv_color())
}

pub fn ring() -> impl IntoAsIR {
    Ring::new(1.0, 0.2).set_post(COLOR, uv_color())
}

pub fn union() -> impl IntoAsIR {
    Combine::from(Union).push(circle()).push(line())
}

pub fn smooth_union() -> impl IntoAsIR {
    Combinator::build()
        .push(Union)
        .push(SmoothUnion::new(DISTANCE, 0.4))
        .push(SmoothUnion::new(GRADIENT_2D, 0.4))
        .push(SmoothUnion::new(UV, 0.4))
        .combine()
        .push(circle())
        .push(line())
}

pub fn kettle_bell() -> impl IntoAsIR {
    Combinator::build()
        .push(Subtraction)
        .push(SmoothSubtraction::new(DISTANCE, 0.4))
        .push(SmoothSubtraction::new(GRADIENT_2D, 0.4))
        .push(SmoothSubtraction::new(UV, 0.4))
        .combine()
        .push(
            Combinator::build()
                .push(Union)
                .push(SmoothUnion::new(DISTANCE, 0.4))
                .push(SmoothUnion::new(GRADIENT_2D, 0.4))
                .push(SmoothUnion::new(UV, 0.4))
                .combine()
                .push(Circle::new(1.0).translate([0.0, -0.5]))
                .push(Ring::new(0.9, 0.15).translate([0.0, 0.25]))
                .push(Capsule::new([1.5, 0.0], 0.2).translate([0.0, -0.5])),
        )
        .push(Capsule::new([1.5, 0.0], 0.2).translate([0.0, -0.5]))
        .gradient_normals()
        .set_post(COLOR, uv_color())
}

pub fn select() -> impl IntoAsIR {
    let id_x = REPEAT_ID_2D.path().push(X).read();
    let id_x_lt = |t: f64| id_x.clone().lt(t);

    let id_y = REPEAT_ID_2D.path().push(Y).read();
    let id_y_lt = |t: f64| id_y.clone().lt(t);

    use elysian_text::glyphs::upper::*;

    Select::new(Infinity)
        .case(
            id_x_lt(1.0).and(id_y_lt(1.0)),
            a().set_post(COLOR, distance_color()),
        )
        .case(
            id_x_lt(2.0).and(id_y_lt(1.0)),
            n().set_post(COLOR, distance_color()),
        )
        .case(id_x_lt(3.0).and(id_y_lt(1.0)), point())
        .case(id_x_lt(4.0).and(id_y_lt(1.0)), chebyshev())
        .case(id_x_lt(5.0).and(id_y_lt(1.0)), raymarched())
        .case(
            id_x_lt(1.0).and(id_y_lt(2.0)),
            sigma().set_post(COLOR, distance_color()),
        )
        .case(
            id_x_lt(2.0).and(id_y_lt(2.0)),
            l().set_post(COLOR, distance_color()),
        )
        .case(
            id_x_lt(3.0).and(id_y_lt(2.0)),
            y().set_post(COLOR, distance_color()),
        )
        .case(
            id_x_lt(4.0).and(id_y_lt(2.0)),
            z().set_post(COLOR, distance_color()),
        )
        .case(
            id_x_lt(5.0).and(id_y_lt(2.0)),
            i().set_post(COLOR, distance_color()),
        )
        .scale(0.35)
        .aspect(ASPECT.prop().read())
        .repeat_clamped([0.8, 2.0], [0.0, 0.0], [4.0, 2.0])
        .translate([-1.6, -1.0])
}

pub fn distance_color() -> Expr {
    Expr::vector4(
        1.0.literal() - DISTANCE.prop().read() * 50.0,
        1.0.literal() - DISTANCE.prop().read() * 50.0,
        1.0.literal() - DISTANCE.prop().read() * 50.0,
        1.0.literal(),
    )
}

pub fn normal_color() -> Expr {
    Expr::vector4(
        NORMAL.path().push(X).read() * 0.5 + 0.5,
        NORMAL.path().push(Y).read() * 0.5 + 0.5,
        NORMAL.path().push(Z).read() * 0.5 + 0.5,
        1.0,
    )
}

pub fn distance_normal_color() -> Expr {
    Expr::vector4(
        (1.0.literal() - DISTANCE.prop().read().abs()) * (NORMAL.path().push(X).read() * 0.5 + 0.5),
        (1.0.literal() - DISTANCE.prop().read().abs()) * (NORMAL.path().push(Y).read() * 0.5 + 0.5),
        (1.0.literal() - DISTANCE.prop().read().abs()) * (NORMAL.path().push(Z).read() * 0.5 + 0.5),
        1.0,
    )
}

pub fn uv_color() -> Expr {
    Expr::vector4(UV.path().push(X).read(), UV.path().push(Y).read(), 0.0, 1.0)
}

pub fn cell_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        CELL_ID.prop().read() * fac,
        CELL_ID.prop().read() * fac,
        CELL_ID.prop().read() * fac,
        1.0,
    )
}

pub fn repeat_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        REPEAT_ID_2D.path().push(X).read().abs() * fac,
        REPEAT_ID_2D.path().push(Y).read().abs() * fac,
        0.0,
        1.0,
    )
}

pub fn raymarched() -> impl IntoAsIR {
    //let projection = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    let projection = Mat4::perspective_infinite_rh(std::f32::consts::PI * 0.5, 1.0, 0.01);
    Raymarch::sphere(
        0.0001,
        100u64,
        projection.inverse(),
        Combine::from(Union)
            .push(
                Point
                    .elongate_axis([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
                    .translate([0.5, 0.5, -2.0])
                    .isosurface(1.0)
                    .manifold()
                    .isosurface(0.2),
            )
            .push(
                Point
                    .elongate_axis([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
                    .translate([-0.5, -0.5, -2.5])
                    .isosurface(1.0)
                    .manifold()
                    .isosurface(0.2),
            )
            .push(
                Point
                    .elongate_axis([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
                    .translate([1.0, -1.5, -3.0])
                    .isosurface(1.0)
                    .manifold()
                    .isosurface(0.2),
            )
            .gradient_normals()
            .set_post(COLOR, uv_color()),
    )
}

pub fn partition() -> impl IntoAsIR {
    let cell_id = CELL_ID.prop().read();
    let cell_id_lt = |t: f64| cell_id.clone().lt(t);

    use elysian_text::glyphs::upper::*;

    Select::new(Infinity)
        .case(cell_id_lt(1.0), h())
        .case(cell_id_lt(2.0), e())
        .case(cell_id_lt(3.0), l())
        .case(cell_id_lt(4.0), l())
        .case(cell_id_lt(5.0), o())
        .case(cell_id_lt(6.0), w())
        .case(cell_id_lt(7.0), o())
        .case(cell_id_lt(8.0), r())
        .case(cell_id_lt(9.0), l())
        .case(cell_id_lt(10.0), d())
        .case(cell_id_lt(11.0), Infinity)
        .scale(0.35)
        .modify()
        .push_pre(voronoi([
            [-2.0, 0.5],
            [-1.0, 1.0],
            [0.0, 1.5],
            [1.0, 1.0],
            [2.0, 0.5],
            [-2.0, -0.5],
            [-1.0, -1.0],
            [0.0, -1.5],
            [1.0, -1.0],
            [2.0, -0.5],
            [0.0, 0.0],
        ]))
        .set_post(COLOR, distance_color())
        .aspect(ASPECT.prop().read())
}

pub fn test_shape() -> impl IntoAsIR {
    text(
        "SPHINX OF\nBLACK QUARTZ,\nJUDGE MY VOW.",
        Align::Center,
        [1.2, 1.2],
        0.35,
    )
    //.isosurface(0.05)
    .scale(0.4)
    .set_post(COLOR, distance_color())
    .aspect(ASPECT.prop().read())
}

pub fn shapes() -> impl IntoIterator<Item = (&'static str, DynAsIR)> {
    [("test_shape", test_shape().as_ir())]
}
