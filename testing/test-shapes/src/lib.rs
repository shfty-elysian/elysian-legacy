use elysian_core::{
    ast::{
        attribute::Attribute::{self, *},
        central_diff_gradient::CentralDiffGradient,
        combine::{Blend, Boolean},
        expr::IntoLiteral,
        field::{Capsule, Circle, IntoField, Point, Ring},
        modify::IntoModify,
        raymarch::{March, Raymarch},
        IntoCombine,
    },
    ir::{as_ir::DynAsIR, module::DynAsModule},
};
use rust_gpu_bridge::glam::Mat4;

pub fn kettle_bell() -> DynAsModule {
    let smooth_union: [DynAsIR; 3] = [
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

    let smooth_subtraction: [DynAsIR; 3] = [
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

    let shape_a: [DynAsModule; 2] = [
        Box::new(
            Circle {
                radius: 1.0.literal(),
            }
            .field()
            .modify()
            .translate([0.0, 0.5].literal()),
        ),
        Box::new(
            Ring {
                radius: 0.9.literal(),
                width: 0.15.literal(),
            }
            .field()
            .modify()
            .translate([0.0, -0.25].literal()),
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
            .modify()
            .translate([0.0, 0.5].literal()),
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
    //let projection = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    let projection = Mat4::perspective_infinite_rh(std::f32::consts::PI * 0.5, 1.0, 0.01);
    Box::new(Raymarch {
        march: March::Sphere {
            epsilon: 0.0001.literal(),
        },
        max_steps: 100u32.literal(),
        projection: projection.literal(),
        inv_projection: projection.inverse().literal(),
        field: Box::new(
            Point
                .field()
                .modify()
                .translate([0.5, 0.5, -2.0].literal())
                .elongate([0.5, 0.0, 0.0].literal(), false)
                .isosurface(1.0.literal())
                .manifold()
                .isosurface(0.2.literal())
                .gradient_normals(),
        ),
    })
}

pub fn shapes() -> [(&'static str, DynAsModule); 2] {
    [("point", point()), ("kettle_bell", kettle_bell())]
}
