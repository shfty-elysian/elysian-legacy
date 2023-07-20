use elysian_core::{
    ast::{
        attribute::Attribute::{self, *},
        central_diff_gradient::CentralDiffGradient,
        combine::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, IntoField, Point, Ring},
        modify::IntoModify,
        raymarch::Raymarch,
        IntoCombine,
    },
    ir::{as_ir::DynAsIR, module::DynAsModule},
};
use rust_gpu_bridge::glam::{Vec2, Vec3};

pub fn kettle_bell() -> DynAsModule {
    let smooth_union: [DynAsIR; 3] = [
        Box::new(Boolean::Union),
        Box::new(Blend::SmoothUnion {
            attr: Distance,
            k: 0.4_f32.literal(),
        }),
        Box::new(Blend::SmoothUnion {
            attr: Gradient,
            k: 0.4_f32.literal(),
        }),
    ];

    let smooth_subtraction: [DynAsIR; 3] = [
        Box::new(Boolean::Subtraction),
        Box::new(Blend::SmoothSubtraction {
            attr: Attribute::Distance,
            k: 0.4_f32.literal(),
        }),
        Box::new(Blend::SmoothSubtraction {
            attr: Attribute::Gradient,
            k: 0.4_f32.literal(),
        }),
    ];

    let shape_a: [DynAsModule; 2] = [
        Box::new(
            Circle {
                radius: 1.0_f32.literal(),
            }
            .field()
            .modify()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9_f32.literal(),
                width: 0.15_f32.literal(),
            }
            .field()
            .modify()
            .translate(Vec2::new(0.0, -0.25).literal()),
        ),
    ];

    let shape_b: [DynAsModule; 2] = [
        Box::new(shape_a.combine(smooth_union)),
        Box::new(
            Capsule {
                dir: Vec2::new(1.5, 0.0).literal(),
                radius: 0.2_f32.literal(),
            }
            .field()
            .modify()
            .translate(Vec2::new(0.0, 0.5).literal()),
        ),
    ];

    let shape_c = shape_b.combine(smooth_subtraction);

    let shape_d = CentralDiffGradient {
        field: Box::new(shape_c),
        epsilon: 0.01.into(),
    };

    let shape_e = shape_d.modify().gradient_normals();

    Box::new(shape_e)
}

pub fn point() -> DynAsModule {
    Box::new(Raymarch {
        step_size: (1.0_f32 / 10.0).literal(),
        max_steps: 100i32.literal(),
        field: Box::new(
            Point
                .field()
                .modify()
                .elongate((Vec3::X * 0.5).literal(), false)
                .isosurface(1.0.literal())
                .manifold()
                .isosurface(0.2.literal())
                .translate((-Vec3::Z * 5.0).literal())
                .gradient_normals(),
        ),
    })
}

pub fn shapes() -> [(&'static str, DynAsModule); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
