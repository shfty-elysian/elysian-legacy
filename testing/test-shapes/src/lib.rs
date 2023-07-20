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
    ir::{as_ir::DynAsIR, ast::GlamF32, module::DynAsModule},
};
use rust_gpu_bridge::glam::{Vec2, Vec3};

pub fn kettle_bell() -> DynAsModule<GlamF32> {
    let smooth_union: [DynAsIR<GlamF32>; 3] = [
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

    let smooth_subtraction: [DynAsIR<GlamF32>; 3] = [
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

    let shape_a: [DynAsModule<GlamF32>; 2] = [
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
    ];

    let shape_b: [DynAsModule<GlamF32>; 2] = [
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

    let shape_e = shape_d.modify().gradient_normals();

    Box::new(shape_e)
}

pub fn point() -> DynAsModule<GlamF32> {
    Box::new(Raymarch::<GlamF32> {
        step_size: (1.0 / 10.0).literal(),
        max_steps: 100.0.literal(),
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

pub fn shapes() -> [(&'static str, DynAsModule<GlamF32>); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
