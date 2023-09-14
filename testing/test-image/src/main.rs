use std::time::Instant;

use elysian::{
    image::{color_to_rgb8, rasterize},
    interpreter::Interpreted,
    ir::{
        ast::COLOR,
        module::{AsModule, Dispatch, EvaluateError, SpecializationData},
    },
    r#static::{include_static_shapes, Precompiled},
    shapes::{color::distance_color, modify::IntoSet, shape::IntoShape},
};
use image::Rgb;
use viuer::Config;

include_static_shapes!();

fn main() -> Result<(), EvaluateError> {
    let shape = test_shapes::pangram()
        .module(&SpecializationData::new_2d())
        .finalize();

    let start = Instant::now();
    let (width, height) = (16, 16);

    let image = rasterize::<Rgb<u8>>(
        Dispatch(vec![
            Box::new(Precompiled(&shape)),
            Box::new(Interpreted(&shape)),
        ]),
        width,
        height,
        color_to_rgb8,
    )?;
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

    Ok(())
}

mod quadtree {
    use elysian::{
        core::number::Number,
        ir::{
            ast::{Struct, Value, DISTANCE, GRADIENT_2D, POSITION_2D, VECTOR2, X, Y},
            module::{Evaluate, EvaluateError, StructIdentifier, CONTEXT},
        },
        shapes::{
            combine::{Combine, Union},
            field::Quad,
            modify::{IntoManifold, IntoTranslate, ASPECT},
            shape::IntoShape,
        },
    };

    pub fn quadtree<'a>(
        shape: &impl Evaluate<'a>,
        [min_x, min_y]: [f64; 2],
        [max_x, max_y]: [f64; 2],
        level: usize,
    ) -> Result<impl IntoShape, EvaluateError> {
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

            shape.evaluate(ctx)
        };

        let [width, height] = [max_x - min_x, max_y - min_y];
        let qsize = [width * 0.25, height * 0.25];
        let [hw, hh] = [width * 0.5, height * 0.5];

        let mut out = Combine::from(Union);

        for (iy, y) in [(0, 1), (1, 3)] {
            for (ix, x) in [(0, 1), (1, 3)] {
                let p @ [x, y] = [min_x + qsize[0] * x as f64, min_y + qsize[1] * y as f64];

                let ctx = sample(x, y)?;
                let d: f64 = ctx.get(&DISTANCE.into()).into();

                if d <= 0.0 {
                    out = out.push(Quad::new(qsize, [GRADIENT_2D]).translate(p).manifold());

                    if level > 1 {
                        let lmin @ [lmin_x, lmin_y] =
                            [min_x + hw * ix as f64, min_y * hh * iy as f64];
                        let lmax = [lmin_x + hw, lmin_y + hh];
                        out = out.push(quadtree(shape, lmin, lmax, level - 1)?);
                    }
                }
            }
        }

        Ok(out)
    }
}
