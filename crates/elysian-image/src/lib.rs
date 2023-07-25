use elysian_shapes::modify::ASPECT;
use image::RgbImage;
use rust_gpu_bridge::glam::Vec4;
use tracing::instrument;

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use elysian_core::ir::{
    ast::{Number, Struct, Value, COLOR, CONTEXT, POSITION_2D, VECTOR2, X, Y},
    module::{AsModule, SpecializationData},
};
use elysian_static::dispatch_shape;

#[instrument]
pub fn rasterize<T>(
    shape: &T,
    spec: &SpecializationData,
    width: u32,
    height: u32,
    scale: f32,
) -> RgbImage
where
    T: AsModule,
{
    let shape = dispatch_shape(shape, spec);

    let indices: Vec<_> = (0..height)
        .into_iter()
        .flat_map(move |y| (0..width).into_iter().map(move |x| (x, y)))
        .collect();

    let chunk_size = (width * height) as usize / num_cpus::get();

    let pixels = indices
        .into_par_iter()
        .chunks(chunk_size)
        .flat_map(|indices| {
            indices
                .into_iter()
                .flat_map(|(x, y)| {
                    let ctx = Struct::new(CONTEXT)
                        .set(
                            POSITION_2D,
                            Value::Struct(
                                Struct::new(VECTOR2)
                                    .set(
                                        X,
                                        (((x as f32 / width as f32) - 0.5) * 2.0 / scale).into(),
                                    )
                                    .set(
                                        Y,
                                        (((y as f32 / height as f32) - 0.5) * -2.0 / scale).into(),
                                    ),
                            ),
                        )
                        .set(
                            ASPECT,
                            Value::Number(Number::Float(width as f64 / height as f64)),
                        );

                    let ctx = shape(ctx);

                    let c: Vec4 = ctx.get(&COLOR).into();
                    [
                        (c.x * 255.0).round() as u8,
                        (c.y * 255.0).round() as u8,
                        (c.z * 255.0).round() as u8,
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect();

    RgbImage::from_vec(width, height, pixels).expect("Failed to create image")
}
