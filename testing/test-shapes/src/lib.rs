use elysian_core::ast::{
    attribute::Attribute::{self, *},
    combinator::{
        Blend, Boolean,
        Combinator::{self, *},
    },
    expr::IntoLiteral,
    field::{Capsule, Circle, IntoField, Point, Ring},
    Elysian, IntoCombine,
};
use rust_gpu_bridge::glam::Vec2;

pub fn kettle_bell() -> Elysian<f32, Vec2> {
    let smooth_union = [
        Combinator::Boolean(Boolean::Union),
        Blend(Blend::SmoothUnion {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Blend(Blend::SmoothUnion {
            attr: Gradient,
            k: 0.4.literal(),
        }),
    ];

    let smooth_subtraction = [
        Combinator::Boolean(Boolean::Subtraction),
        Combinator::Blend(Blend::SmoothSubtraction {
            attr: Attribute::Distance,
            k: 0.4.literal(),
        }),
        Combinator::Blend(Blend::SmoothSubtraction {
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

pub fn point() -> Elysian<f32, Vec2> {
    Elysian::Field {
        pre_modifiers: Default::default(),
        field: Box::new(Point),
        post_modifiers: Default::default(),
    }
}

pub fn shapes() -> [(&'static str, Elysian<f32, Vec2>); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
