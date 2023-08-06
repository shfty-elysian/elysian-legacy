use elysian_core::{
    ast::{
        combine::{Combinator, Combine},
        expr::{Expr, IntoExpr, IntoLiteral, IntoRead},
        modify::IntoModify,
        select::Switch,
    },
    ir::{
        ast::{COLOR, DISTANCE, GRADIENT_2D, NORMAL, UV, X, Y, Z},
        module::{DynAsIR, IntoAsIR},
    },
};
use elysian_shapes::{
    combine::{Displace, Sided, SidedProp, SmoothSubtraction, SmoothUnion, Subtraction, Union},
    field::{Capsule, Chebyshev, Circle, Line, Point, Ring},
    modify::{
        BoundType, ClampMode, IntoAspect, IntoBasisBound, IntoDistanceBound, IntoElongate,
        IntoGradientNormals, IntoIsosurface, IntoManifold, IntoMirror, IntoRepeat, IntoSet,
        IntoTranslate, ASPECT, REPEAT_ID_2D,
    },
    partition::PARTITION_ID,
    raymarch::Raymarch,
    scale::IntoScale,
};
use elysian_text::glyphs::{greek::sigma, upper::*};
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

pub fn quad(extent: impl IntoExpr) -> impl IntoAsIR {
    Combinator::build()
        .push(Sided::left())
        .push(Displace::new(DISTANCE))
        .push(SidedProp::new(GRADIENT_2D, false))
        .combine()
        .push(Point.basis_bound(BoundType::Lower, [0.0, 0.0]))
        .push(Chebyshev.distance_bound(BoundType::Upper, 0.0))
        .translate(extent)
        .mirror_basis([1.0, 1.0])
        .gradient_normals()
        .set_post(COLOR, normal_color())
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
    let id_y = REPEAT_ID_2D.path().push(Y).read();

    Switch::new(point())
        .case(
            id_x.clone().lt(1.0).and(id_y.clone().lt(1.0)),
            a().set_post(COLOR, distance_color()),
        )
        .case(
            id_x.clone().lt(2.0).and(id_y.clone().lt(1.0)),
            n().set_post(COLOR, distance_color()),
        )
        .case(id_x.clone().lt(3.0).and(id_y.clone().lt(1.0)), point())
        .case(id_x.clone().lt(4.0).and(id_y.clone().lt(1.0)), chebyshev())
        .case(id_x.clone().lt(5.0).and(id_y.clone().lt(1.0)), raymarched())
        .case(
            id_x.clone().lt(1.0).and(id_y.clone().lt(2.0)),
            sigma().set_post(COLOR, distance_color()),
        )
        .case(
            id_x.clone().lt(2.0).and(id_y.clone().lt(2.0)),
            l().set_post(COLOR, distance_color()),
        )
        .case(
            id_x.clone().lt(3.0).and(id_y.clone().lt(2.0)),
            y().set_post(COLOR, distance_color()),
        )
        .case(
            id_x.clone().lt(4.0).and(id_y.clone().lt(2.0)),
            z().set_post(COLOR, distance_color()),
        )
        .case(
            id_x.clone().lt(5.0).and(id_y.clone().lt(2.0)),
            i().set_post(COLOR, distance_color()),
        )
        .scale(0.35)
        .aspect(ASPECT.prop().read())
        .repeat_clamped([0.8, 2.0], [0.0, 0.0], [4.0, 2.0])
        .translate([-1.6, -1.0])
}

pub fn distance_color() -> Expr {
    Expr::vector4(
        1.0.literal() - DISTANCE.prop().read().abs(),
        1.0.literal() - DISTANCE.prop().read().abs(),
        1.0.literal() - DISTANCE.prop().read().abs(),
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

pub fn partition_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        PARTITION_ID.prop().read() * fac,
        PARTITION_ID.prop().read() * fac,
        PARTITION_ID.prop().read() * fac,
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
                    .elongate([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
                    .translate([0.5, 0.5, -2.0])
                    .isosurface(1.0)
                    .manifold()
                    .isosurface(0.2),
            )
            .push(
                Point
                    .elongate([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
                    .translate([-0.5, -0.5, -2.5])
                    .isosurface(1.0)
                    .manifold()
                    .isosurface(0.2),
            )
            .push(
                Point
                    .elongate([0.5, 0.0, 0.0], ClampMode::Dir, ClampMode::Dir)
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
    Switch::new(point())
        .case(
            PARTITION_ID.prop().read().lt(1.0),
            a().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(2.0),
            k().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(3.0),
            l().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(4.0),
            w().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(5.0),
            x().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(6.0),
            z().set_post(COLOR, distance_color()),
        )
        .case(
            PARTITION_ID.prop().read().lt(7.0),
            y().set_post(COLOR, distance_color()),
        )
        .scale(0.35)
        .modify()
        .push_pre(
            [
                [0.0, 0.0],
                [-1.5, -0.75],
                [0.0, -1.5],
                [1.5, -0.75],
                [1.5, 0.75],
                [0.0, 1.5],
                [-1.5, 0.75],
            ]
            .into_iter()
            .enumerate()
            .fold(Combine::from(Union), |acc, (i, next)| {
                acc.push(Point.translate(next).set_pre(PARTITION_ID, i as f32))
            }),
        )
        .set_post(COLOR, distance_color())
        .aspect(ASPECT.prop().read())
}

pub fn test_shape() -> impl IntoAsIR {
    partition()
}

pub fn shapes() -> impl IntoIterator<Item = (&'static str, DynAsIR)> {
    [
        ("point", point().as_ir()),
        ("chebyshev", chebyshev().as_ir()),
        ("line", line().as_ir()),
        ("quad", quad([1.0, 0.5]).as_ir()),
        ("circle", circle().as_ir()),
        ("capsule", capsule().as_ir()),
        ("ring", ring().as_ir()),
        ("union", union().as_ir()),
        ("smooth_union", smooth_union().as_ir()),
        ("kettle_bell", kettle_bell().as_ir()),
        ("t", t().as_ir()),
        ("raymarched", raymarched().as_ir()),
        ("select", select().as_ir()),
        ("partition", partition().as_ir()),
    ]
}
