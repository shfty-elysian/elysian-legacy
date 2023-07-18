use elysian::{
    core::ast::{
        attribute::Attribute::{self, *},
        combinator::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, Ring},
        IntoCombine,
    },
    syn::{elysian_to_syn, prettyplease},
};
use elysian_core::{
    ast::field::IntoField,
    ir::{as_ir::DynAsIR, ast::GlamF32},
};
use rust_gpu_bridge::glam::Vec2;

fn main() {
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
