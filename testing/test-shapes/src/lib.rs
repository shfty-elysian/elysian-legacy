use elysian_core::{
    ast::{
        attribute::Attribute::{self, *},
        combine::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, Field, IntoField, Point, Ring},
        IntoCombine,
    },
    ir::{as_ir::DynAsIR, ast::GlamF32, module::DynAsModule},
};
use rust_gpu_bridge::glam::Vec2;

pub fn kettle_bell() -> DynAsModule<GlamF32, 2> {
    let smooth_union: [DynAsIR<GlamF32, 2>; 3] = [
        Box::new(Boolean::Union),
        Box::new(Blend::SmoothUnion {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Box::new(Blend::SmoothUnion {
            attr: Gradient,
            k: 0.4.literal(),
        }),
    ];

    let smooth_subtraction: [DynAsIR<GlamF32, 2>; 3] = [
        Box::new(Boolean::Subtraction),
        Box::new(Blend::SmoothSubtraction {
            attr: Attribute::Distance,
            k: 0.4.literal(),
        }),
        Box::new(Blend::SmoothSubtraction {
            attr: Attribute::Gradient,
            k: 0.4.literal(),
        }),
    ];

    let shape_a: [DynAsModule<GlamF32, 2>; 2] = [
        Box::new(
            Circle {
                radius: 1.0.literal(),
            }
            .field()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .field()
            .translate(Vec2::new(0.0, -0.25).literal()),
        ),
    ];

    let shape_b: [DynAsModule<GlamF32, 2>; 2] = [
        Box::new(shape_a.combine(smooth_union)),
        Box::new(
            Capsule {
                dir: Vec2::new(1.5, 0.0).literal(),
                radius: 0.2.literal(),
            }
            .field()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
    ];

    let shape: DynAsModule<GlamF32, 2> = Box::new(shape_b.combine(smooth_subtraction));

    shape
}

pub fn point() -> DynAsModule<GlamF32, 2> {
    Box::new(Field {
        pre_modifiers: Default::default(),
        field: Box::new(Point),
        post_modifiers: Default::default(),
    })
}

pub fn shapes() -> [(&'static str, DynAsModule<GlamF32, 2>); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
