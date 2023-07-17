use elysian::{
    core::ast::{
        attribute::Attribute::{self, *},
        combinator::{Blend, Boolean, Combinator},
        expr::IntoLiteral,
        field::{Capsule, Circle, Ring},
        IntoCombine,
    },
    syn::{elysian_to_syn, prettyplease},
};
use elysian_core::ast::field::IntoField;
use rust_gpu_bridge::glam::Vec2;

fn main() {
    let smooth_union = [
        Combinator::Boolean(Boolean::Union),
        Combinator::Blend(Blend::SmoothUnion {
            attr: Distance,
            k: 0.4.literal(),
        }),
        Combinator::Blend(Blend::SmoothUnion {
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

    let shape = [
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
    .combine(smooth_subtraction);

    let source = elysian_to_syn(&shape, "test");
    let source = prettyplease::unparse(&source);
    println!("{source:}");
}
