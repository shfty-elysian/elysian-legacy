use rust_gpu_bridge::glam::Vec2;

use elysian::{
    core::ast::{
        alias::{Capsule, Circle, Ring},
        attribute::Attribute::{self, *},
        combinator::{Blend, Boolean, Combinator},
        expand::Expand,
        expr::IntoLiteral,
        to_glam::ToGlam,
        IntoAlias, IntoCombine,
    },
    syn::{elysian_to_syn, prettyplease},
};

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
            .alias()
            .expand()
            .translate([0.0, 0.5].literal()),
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .alias()
            .expand()
            .translate([0.0, -0.25].literal()),
        ]
        .combine(smooth_union),
        Capsule {
            dir: [1.5, 0.0].literal(),
            radius: 0.2.literal(),
        }
        .alias()
        .expand()
        .translate([0.0, 0.5].literal()),
    ]
    .combine(smooth_subtraction);

    let foo = elysian_to_syn::<f32, Vec2>(&shape.expand().to_glam(), "test");
    let foo = prettyplease::unparse(&foo);
    println!("{foo:}");
}
