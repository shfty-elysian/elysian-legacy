use elysian_core::{
    ast::{
        attribute::Attribute::{self, *},
        combinator::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, IntoField, Point, Ring},
        Elysian, IntoCombine,
    },
    ir::{as_ir::AsIR, ast::GlamF32},
};
use rust_gpu_bridge::glam::Vec2;

pub fn kettle_bell() -> Elysian<GlamF32, 2> {
    let smooth_union: [Box<dyn AsIR<GlamF32, 2>>; 3] = [
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

    let smooth_subtraction: [Box<dyn AsIR<GlamF32, 2>>; 3] = [
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

    [
        [
            Circle {
                radius: 1.0.literal(),
            }
            .field()
            .translate(Vec2::new(0.0, 0.5).literal()),
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .field()
            .translate(Vec2::new(0.0, -0.25).literal()),
        ]
        .combine(smooth_union),
        Capsule {
            dir: Vec2::new(1.5, 0.0).literal(),
            radius: 0.2.literal(),
        }
        .field()
        .translate(Vec2::new(0.0, 0.5).literal()),
    ]
    .combine(smooth_subtraction)
}

pub fn point() -> Elysian<GlamF32, 2> {
    Elysian::Field {
        pre_modifiers: Default::default(),
        field: Box::new(Point),
        post_modifiers: Default::default(),
    }
}

pub fn shapes() -> [(&'static str, Elysian<GlamF32, 2>); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
