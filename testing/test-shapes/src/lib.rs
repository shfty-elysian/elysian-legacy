use elysian_core::{
    ast::{
        combine::IntoCombine,
        expr::{Expr, IntoLiteral, IntoRead},
        select::IntoSelect,
    },
    ir::{
        ast::{COLOR, DISTANCE, GRADIENT_2D, NORMAL, UV, X, Y, Z},
        module::{AsIR, DynAsIR, PropertyIdentifier},
    },
};
use elysian_shapes::{
    combine::{Displace, Sided, SidedProp, SmoothSubtraction, SmoothUnion, Subtraction, Union},
    field::{Capsule, Chebyshev, Circle, Line, LineMode, Point, Ring},
    modify::{
        BoundType, ClampMode, IntoAspect, IntoBasisBound, IntoDistanceBound, IntoElongate,
        IntoGradientNormals, IntoIsosurface, IntoManifold, IntoMirror, IntoRepeat, IntoSet,
        IntoTranslate, MirrorMode, ASPECT, REPEAT_ID_2D,
    },
    raymarch::{March, Raymarch},
    scale::IntoScale,
};
use elysian_text::glyphs::upper::*;
use rust_gpu_bridge::glam::Mat4;

pub fn point() -> DynAsIR {
    Box::new(
        Point
            .gradient_normals()
            .set_post(COLOR.into(), distance_normal_color()),
    )
}

pub fn chebyshev() -> DynAsIR {
    Box::new(
        Chebyshev
            .gradient_normals()
            .set_post(COLOR.into(), distance_normal_color()),
    )
}

pub fn line() -> DynAsIR {
    Box::new(
        Line {
            dir: [1.0, 0.0].literal(),
            mode: LineMode::Centered,
        }
        .set_post(COLOR.into(), uv_color()),
    )
}

fn quad() -> DynAsIR {
    let extent = [1.0, 0.5].literal();

    Box::new(
        [
            Box::new(Point.basis_bound(BoundType::Lower, Expr::Literal([0.0, 0.0].into())))
                as Box<dyn AsIR>,
            Box::new(Chebyshev.distance_bound(BoundType::Upper, Expr::Literal(0.0.into()))),
        ]
        .combine([
            Box::new(Sided { flip: false }) as DynAsIR,
            Box::new(Displace {
                prop: DISTANCE.into(),
            }),
            Box::new(SidedProp {
                prop: GRADIENT_2D.into(),
                flip: false,
            }),
        ])
        .translate(extent.clone())
        .mirror(MirrorMode::Basis([1.0, 1.0].literal()))
        .gradient_normals()
        .set_post(COLOR.into(), normal_color()),
    )
}

pub fn circle() -> DynAsIR {
    Box::new(
        Circle {
            radius: 0.5.literal(),
        }
        .gradient_normals()
        .set_post(COLOR.into(), uv_color()),
    )
}

pub fn capsule() -> DynAsIR {
    Box::new(
        Capsule {
            dir: [1.5, 0.0].literal(),
            radius: 0.5.literal(),
        }
        .set_post(COLOR.into(), uv_color()),
    )
}

pub fn ring() -> DynAsIR {
    Box::new(
        Ring {
            radius: 1.0.literal(),
            width: 0.2.literal(),
        }
        .set_post(COLOR.into(), uv_color()),
    )
}

pub fn union() -> DynAsIR {
    Box::new([circle(), line()].combine([Box::new(Union::default()) as DynAsIR]))
}

pub fn smooth_union() -> DynAsIR {
    Box::new([circle(), line()].combine([
        Box::new(Union::default()) as DynAsIR,
        Box::new(SmoothUnion {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: UV.into(),
            k: 0.4.literal(),
        }),
    ]))
}

pub fn kettle_bell() -> DynAsIR {
    let smooth_union: [DynAsIR; 4] = [
        Box::new(Union::default()),
        Box::new(SmoothUnion {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: UV.into(),
            k: 0.4.literal(),
        }),
    ];

    let smooth_subtraction: [DynAsIR; 4] = [
        Box::new(Subtraction),
        Box::new(SmoothSubtraction {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothSubtraction {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothSubtraction {
            prop: UV.into(),
            k: 0.4.literal(),
        }),
    ];

    let shape_a: [DynAsIR; 2] = [
        Box::new(
            Circle {
                radius: 1.0.literal(),
            }
            .translate([0.0, -0.5].literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .translate([0.0, 0.25].literal()),
        ),
    ];

    let shape_b: [DynAsIR; 2] = [
        Box::new(shape_a.combine(smooth_union)),
        Box::new(
            Capsule {
                dir: [1.5, 0.0].literal(),
                radius: 0.2.literal(),
            }
            .translate([0.0, -0.5].literal()),
        ),
    ];

    let shape_c = shape_b.combine(smooth_subtraction);

    let shape_d = shape_c
        .gradient_normals()
        .set_post(COLOR.into(), uv_color());

    Box::new(shape_d)
}

pub fn select() -> DynAsIR {
    Box::new(
        [
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(1.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(1.0.literal())),
                point(),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(2.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(1.0.literal())),
                chebyshev(),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(3.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(1.0.literal())),
                line(),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(4.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(1.0.literal())),
                quad(),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(5.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(1.0.literal())),
                Box::new(z().set_post(COLOR.into(), distance_color())),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(1.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(2.0.literal())),
                Box::new(e().set_post(COLOR.into(), distance_color())),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(2.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(2.0.literal())),
                Box::new(y().set_post(COLOR.into(), distance_color())),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(3.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(2.0.literal())),
                Box::new(t().set_post(COLOR.into(), distance_color())),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(4.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(2.0.literal())),
                Box::new(a().set_post(COLOR.into(), distance_color())),
            ),
            (
                [REPEAT_ID_2D.into(), X.into()]
                    .read()
                    .lt(5.0.literal())
                    .and([REPEAT_ID_2D.into(), Y.into()].read().lt(2.0.literal())),
                raymarched(),
            ),
        ]
        .select(point())
        .scale(0.35.literal())
        .aspect(PropertyIdentifier(ASPECT).read())
        .repeat(
            [0.8, 2.0].literal(),
            Some(([0.0, 0.0].literal(), [4.0, 2.0].literal())),
        )
        .translate([-1.6, -1.0].literal()),
    )
}

pub fn distance_color() -> Expr {
    Expr::vector4(
        1.0.literal() - PropertyIdentifier(DISTANCE).read().abs(),
        1.0.literal() - PropertyIdentifier(DISTANCE).read().abs(),
        1.0.literal() - PropertyIdentifier(DISTANCE).read().abs(),
        1.0.literal(),
    )
}

pub fn normal_color() -> Expr {
    Expr::vector4(
        [NORMAL.into(), X.into()].read() * 0.5.literal() + 0.5.literal(),
        [NORMAL.into(), Y.into()].read() * 0.5.literal() + 0.5.literal(),
        [NORMAL.into(), Z.into()].read() * 0.5.literal() + 0.5.literal(),
        1.0.literal(),
    )
}

pub fn distance_normal_color() -> Expr {
    Expr::vector4(
        (1.0.literal() - PropertyIdentifier(DISTANCE).read().abs())
            * ([NORMAL.into(), X.into()].read() * 0.5.literal() + 0.5.literal()),
        (1.0.literal() - PropertyIdentifier(DISTANCE).read().abs())
            * ([NORMAL.into(), Y.into()].read() * 0.5.literal() + 0.5.literal()),
        (1.0.literal() - PropertyIdentifier(DISTANCE).read().abs())
            * ([NORMAL.into(), Z.into()].read() * 0.5.literal() + 0.5.literal()),
        1.0.literal(),
    )
}

pub fn uv_color() -> Expr {
    Expr::vector4(
        [UV.into(), X.into()].read(),
        [UV.into(), Y.into()].read(),
        0.0.literal(),
        1.0.literal(),
    )
}

pub fn repeat_id_color(count: usize) -> Expr {
    let fac = 1.0 / count as f64;
    Expr::vector4(
        [REPEAT_ID_2D.into(), X.into()].read().abs() * fac.literal(),
        [REPEAT_ID_2D.into(), Y.into()].read().abs() * fac.literal(),
        0.0.literal(),
        1.0.literal(),
    )
}

pub fn raymarched() -> DynAsIR {
    //let projection = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    let projection = Mat4::perspective_infinite_rh(std::f32::consts::PI * 0.5, 1.0, 0.01);
    Box::new(Raymarch {
        march: March::Sphere {
            epsilon: 0.0001.literal(),
        },
        max_steps: 100u32.literal(),
        inv_projection: projection.inverse().literal(),
        field: Box::new(
            [
                Box::new(
                    Point
                        .elongate([0.5, 0.0, 0.0].literal(), ClampMode::Dir, ClampMode::Dir)
                        .translate([0.5, 0.5, -2.0].literal())
                        .isosurface(1.0.literal())
                        .manifold()
                        .isosurface(0.2.literal()),
                ) as Box<dyn AsIR>,
                Box::new(
                    Point
                        .elongate([0.5, 0.0, 0.0].literal(), ClampMode::Dir, ClampMode::Dir)
                        .translate([-0.5, -0.5, -2.5].literal())
                        .isosurface(1.0.literal())
                        .manifold()
                        .isosurface(0.2.literal()),
                ),
                Box::new(
                    Point
                        .elongate([0.5, 0.0, 0.0].literal(), ClampMode::Dir, ClampMode::Dir)
                        .translate([1.0, -1.5, -3.0].literal())
                        .isosurface(1.0.literal())
                        .manifold()
                        .isosurface(0.2.literal()),
                ),
            ]
            .combine([Box::new(Union::default()) as DynAsIR])
            .gradient_normals()
            .set_post(COLOR.into(), uv_color()),
        ),
    })
}

pub fn test_shape() -> DynAsIR {
    select()
}

pub fn shapes() -> impl IntoIterator<Item = (&'static str, DynAsIR)> {
    [
        ("point", point()),
        ("chebyshev", chebyshev()),
        ("line", line()),
        ("quad", quad()),
        ("circle", circle()),
        ("capsule", capsule()),
        ("ring", ring()),
        ("union", union()),
        ("smooth_union", smooth_union()),
        ("kettle_bell", kettle_bell()),
        ("t", t()),
        ("raymarched", raymarched()),
        ("select", select()),
    ]
}
