use std::time::Instant;

use elysian::{
    image::{color_to_rgb8, rasterize},
    ir::{
        ast::COLOR,
        module::{AsModule, SpecializationData},
    },
    r#static::include_static_shapes,
    shapes::{color::distance_color, modify::IntoSet, shape::IntoShape},
};
use image::Rgb;
use viuer::Config;

include_static_shapes!();

fn main() {
    let shape = test_shapes::test_shape()
        .module(&SpecializationData::new_2d())
        .finalize();

    let shape = quadtree::quadtree(&shape, [-1.0, -1.0], [1.0, 1.0], 4)
        .shape()
        .set_post(COLOR, distance_color(10.0))
        .module(&SpecializationData::new_2d());

    let start = Instant::now();
    let (width, height) = (16, 16);

    let image = rasterize::<Rgb<u8>>(shape, width, height, color_to_rgb8);
    let duration = Instant::now().duration_since(start);

    viuer::print(
        &image.into(),
        &Config {
            transparent: false,
            absolute_offset: false,
            x: 0,
            y: 0,
            restore_cursor: false,
            width: None,
            height: None,
            truecolor: true,
            use_kitty: true,
            use_iterm: true,
        },
    )
    .unwrap();

    println!("Rasterize took {duration:?}");
}

mod quadtree {
    use elysian::{
        core::number::Number,
        ir::{
            ast::{Struct, Value, DISTANCE, GRADIENT_2D, POSITION_2D, VECTOR2, X, Y},
            module::{Module, StructIdentifier, CONTEXT},
        },
        r#static::dispatch_module,
        shapes::{
            combine::{Combine, Union},
            field::Quad,
            modify::{IntoManifold, IntoTranslate, ASPECT},
            shape::IntoShape,
        },
    };

    pub fn quadtree(module: &Module, min: [f64; 2], max: [f64; 2], level: usize) -> impl IntoShape {
        let eval = dispatch_module(module);

        let sample = |x: f64, y: f64| {
            let ctx = Struct::new(StructIdentifier(CONTEXT))
                .set(
                    POSITION_2D.into(),
                    Value::Struct(
                        Struct::new(StructIdentifier(VECTOR2))
                            .set(X.into(), x.into())
                            .set(Y.into(), y.into()),
                    ),
                )
                .set(ASPECT.into(), Value::Number(Number::Float(1.0)));

            eval(ctx)
        };

        let size = [max[0] - min[0], max[1] - min[1]];
        let qsize = [size[0] * 0.25, size[1] * 0.25];
        let hsize = [size[0] * 0.5, size[1] * 0.5];

        let mut out = Combine::from(Union);

        for (iy, y) in [(0, 1), (1, 3)] {
            for (ix, x) in [(0, 1), (1, 3)] {
                let p = [min[0] + qsize[0] * x as f64, min[1] + qsize[1] * y as f64];

                let ctx = sample(p[0], p[1]);
                let d: f64 = ctx.get(&DISTANCE.into()).into();

                if d <= 0.0 {
                    out = out.push(Quad::new(qsize, [GRADIENT_2D]).translate(p).manifold());

                    if level > 1 {
                        let lmin = [min[0] + hsize[0] * ix as f64, min[1] + hsize[1] * iy as f64];
                        let lmax = [lmin[0] + hsize[0], lmin[1] + hsize[1]];
                        out = out.push(quadtree(module, lmin, lmax, level - 1));
                    }
                }
            }
        }

        out
    }
}
