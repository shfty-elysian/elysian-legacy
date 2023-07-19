use elysian::{
    core::ast::{
        attribute::Attribute::{self, *},
        combine::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, Ring},
        IntoCombine,
    },
    syn::{elysian_to_syn, prettyplease},
};
use elysian_core::{
    ast::{central_diff_gradient::CentralDiffGradient, field::IntoField, modify::IntoModify},
    ir::{as_ir::DynAsIR, ast::GlamF32, module::DynAsModule},
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

    let shape_a: [DynAsModule<GlamF32, 2>; 3] = [
        Box::new(
            Circle {
                radius: 1.0.literal(),
            }
            .field()
            .modify()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .field()
            .modify()
            .translate(Vec2::new(0.0, -0.25).literal()),
        ),
        Box::new(
            Ring {
                radius: 0.45.literal(),
                width: 0.075.literal(),
            }
            .field()
            .modify()
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
            .modify()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
    ];

    let shape_c = shape_b.combine(smooth_subtraction);

    let shape_d = CentralDiffGradient {
        field: Box::new(shape_c),
        epsilon: 0.01,
    };

    let source = elysian_to_syn(&shape_d, "test");
    let source = prettyplease::unparse(&source);
    println!("{source:}");
}
