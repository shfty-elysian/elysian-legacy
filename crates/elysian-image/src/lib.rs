use image::RgbImage;
use tracing::instrument;

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use elysian_core::ir::{
    ast::{Number, Struct, Value, DISTANCE, NORMAL, POSITION_2D},
    module::{AsModule, SpecializationData},
};
use elysian_syn::static_shapes::dispatch_shape_f32;

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
    let shape = dispatch_shape_f32(shape, spec);

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
                    let ctx = Struct::default().set(
                        POSITION_2D,
                        Value::Vector2(
                            (((x as f32 / width as f32) - 0.5) * 2.0 / scale).into(),
                            (((y as f32 / height as f32) - 0.5) * 2.0 / scale).into(),
                        ),
                    );

                    let ctx = shape(ctx);

                    let Value::Number(Number::Float(d)) = ctx.get(&DISTANCE) else {
                        panic!("Value is not a Float Number");
                    };

                    let Value::Vector3(Number::Float(x), Number::Float(y), Number::Float(z)) = ctx.get(&NORMAL) else {
                        panic!("Value is not a Float Vector3")
                    };

                    if d >= 0.0 && d <= 4.0 / width as f32 {
                        [255, 255, 255]
                    } else if d <= 0.0 {
                        [
                            ((x * 0.5 + 0.5) * 255.0).round() as u8,
                            ((y * 0.5 + 0.5) * 255.0).round() as u8,
                            ((z * 0.5 + 0.5) * 255.0).round() as u8,
                        ]
                    } else {
                        [
                            ((x * 0.5 + 0.5) * 127.0).round() as u8,
                            ((y * 0.5 + 0.5) * 127.0).round() as u8,
                            0,
                        ]
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    RgbImage::from_vec(width, height, pixels).expect("Failed to create image")
}
