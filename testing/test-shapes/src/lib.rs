use elysian_core::{
    ast::{
        combine::IntoCombine,
        expr::{Expr, IntoLiteral, IntoRead},
        field::IntoField,
        modify::IntoModify,
    },
    ir::{
        as_ir::{AsIR, DynAsIR},
        ast::{COLOR, DISTANCE, GRADIENT_2D, NORMAL, X, Y, Z},
        module::{AsModule, DynAsModule, PropertyIdentifier},
    },
};
use elysian_shapes::{
    combine::{SmoothSubtraction, SmoothUnion, Subtraction, Union},
    field::{Capsule, Circle, Line, Point, Ring},
    modify::{
        IntoAspect, IntoElongate, IntoGradientNormals, IntoIsosurface, IntoManifold, IntoSet,
        IntoTranslate, ASPECT,
    },
    raymarch::{March, Raymarch},
};
use rust_gpu_bridge::glam::Mat4;

pub fn point() -> DynAsModule {
    Box::new(
        Point
            .field()
            .set(COLOR.into(), distance_color())
            .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn circle() -> DynAsModule {
    Box::new(
        Circle {
            radius: 0.5.literal(),
        }
        .field()
        .set(COLOR.into(), distance_color())
        .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn line() -> DynAsModule {
    Box::new(
        Line {
            dir: [1.0, 0.0].literal(),
        }
        .field()
        .set(COLOR.into(), distance_color())
        .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn capsule() -> DynAsModule {
    Box::new(
        Capsule {
            dir: [1.5, 0.0].literal(),
            radius: 0.5.literal(),
        }
        .field()
        .set(COLOR.into(), distance_color())
        .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn ring() -> DynAsModule {
    Box::new(
        Ring {
            radius: 1.0.literal(),
            width: 0.2.literal(),
        }
        .field()
        .set(COLOR.into(), distance_color())
        .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn union() -> DynAsModule {
    Box::new([circle(), line()].combine([Box::new(Union) as Box<dyn AsIR>]))
}

pub fn smooth_union() -> DynAsModule {
    Box::new([circle(), line()].combine([
        Box::new(Union) as Box<dyn AsIR>,
        Box::new(SmoothUnion {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
    ]))
}

pub fn kettle_bell() -> DynAsModule {
    let smooth_union: [DynAsIR; 3] = [
        Box::new(Union),
        Box::new(SmoothUnion {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
    ];

    let smooth_subtraction: [DynAsIR; 3] = [
        Box::new(Subtraction),
        Box::new(SmoothSubtraction {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothSubtraction {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
    ];

    let shape_a: [DynAsModule; 2] = [
        Box::new(
            Circle {
                radius: 1.0.literal(),
            }
            .field()
            .translate([0.0, -0.5].literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .field()
            .translate([0.0, 0.25].literal()),
        ),
    ];

    let shape_b: [DynAsModule; 2] = [
        Box::new(shape_a.combine(smooth_union)),
        Box::new(
            Capsule {
                dir: [1.5, 0.0].literal(),
                radius: 0.2.literal(),
            }
            .field()
            .translate([0.0, -0.5].literal()),
        ),
    ];

    let shape_c = shape_b.combine(smooth_subtraction);

    let shape_d = shape_c
        .modify()
        .gradient_normals()
        .set(COLOR.into(), distance_normal_color())
        .aspect(Expr::Read(vec![ASPECT.into()]));

    Box::new(shape_d)
}

pub fn distance_color() -> Expr {
    Expr::vector4(
        1.0.literal() - PropertyIdentifier(DISTANCE).read(),
        1.0.literal() - PropertyIdentifier(DISTANCE).read(),
        1.0.literal() - PropertyIdentifier(DISTANCE).read(),
        1.0.literal(),
    )
}

pub fn distance_normal_color() -> Expr {
    Expr::vector4(
        (1.0.literal() - PropertyIdentifier(DISTANCE).read())
            * ([NORMAL.into(), X.into()].read() * 0.5.literal() + 0.5.literal()),
        (1.0.literal() - PropertyIdentifier(DISTANCE).read())
            * ([NORMAL.into(), Y.into()].read() * 0.5.literal() + 0.5.literal()),
        (1.0.literal() - PropertyIdentifier(DISTANCE).read())
            * ([NORMAL.into(), Z.into()].read() * 0.5.literal() + 0.5.literal()),
        1.0.literal(),
    )
}

pub fn raymarched() -> DynAsModule {
    //let projection = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    let projection = Mat4::perspective_infinite_rh(std::f32::consts::PI * 0.5, 1.0, 0.01);
    Box::new(
        Raymarch {
            march: March::Sphere {
                epsilon: 0.0001.literal(),
            },
            max_steps: 100u32.literal(),
            inv_projection: projection.inverse().literal(),
            field: Box::new(
                [
                    Box::new(
                        Point
                            .field()
                            .translate([0.5, 0.5, -2.0].literal())
                            .elongate([0.5, 0.0, 0.0].literal(), false)
                            .isosurface(1.0.literal())
                            .manifold()
                            .isosurface(0.2.literal()),
                    ) as Box<dyn AsModule>,
                    Box::new(
                        Point
                            .field()
                            .translate([-0.5, -0.5, -2.5].literal())
                            .elongate([0.5, 0.0, 0.0].literal(), false)
                            .isosurface(1.0.literal())
                            .manifold()
                            .isosurface(0.2.literal()),
                    ),
                    Box::new(
                        Point
                            .field()
                            .translate([1.0, -1.5, -3.0].literal())
                            .elongate([0.5, 0.0, 0.0].literal(), false)
                            .isosurface(1.0.literal())
                            .manifold()
                            .isosurface(0.2.literal()),
                    ),
                ]
                .combine([Box::new(Union) as Box<dyn AsIR>])
                .gradient_normals()
                .set(COLOR.into(), distance_normal_color()),
            ),
        }
        .modify()
        .aspect(Expr::Read(vec![ASPECT.into()])),
    )
}

pub fn test_shape() -> DynAsModule {
    raymarched()
}

pub fn shapes() -> [(&'static str, DynAsModule); 8] {
    [
        ("point", point()),
        ("line", line()),
        ("capsule", capsule()),
        ("ring", ring()),
        ("union", union()),
        ("smooth_union", smooth_union()),
        ("kettle_bell", kettle_bell()),
        ("raymarched", raymarched()),
    ]
}
