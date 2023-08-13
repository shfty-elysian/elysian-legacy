//! Rasterize a 2D Elysian field into an image

use std::fmt::Debug;

use elysian_shapes::modify::ASPECT;
use image::{ImageBuffer, Pixel};
use rust_gpu_bridge::glam::Vec4;

use elysian_core::number::Number;
use elysian_ir::{
    ast::{Struct, Value, COLOR, DISTANCE, POSITION_2D, VECTOR2, X, Y},
    module::{Module, StructIdentifier, CONTEXT},
};
use elysian_static::dispatch_module;

pub fn distance_to_luma_8(ctx: Struct) -> Vec<u8> {
    let d: f64 = ctx.get(&DISTANCE.into()).into();
    let d = (1.0 - d) * 255.0;
    vec![d as u8]
}

pub fn distance_to_luma_32(ctx: Struct) -> Vec<f32> {
    let d: f64 = ctx.get(&DISTANCE.into()).into();
    let d = (1.0 - d) * 255.0;
    vec![d as f32]
}

pub fn color_to_luma_8(ctx: Struct) -> Vec<u8> {
    let c: Vec4 = ctx.get(&COLOR.into()).into();
    let c = c * 255.0;
    vec![c.x as u8]
}

pub fn color_to_luma_a8(ctx: Struct) -> Vec<u8> {
    let c: Vec4 = ctx.get(&COLOR.into()).into();
    let c = c * 255.0;
    vec![c.x as u8, c.w as u8]
}

pub fn color_to_rgb8(ctx: Struct) -> Vec<u8> {
    let c: Vec4 = ctx.get(&COLOR.into()).into();
    let c = c * 255.0;
    vec![c.x as u8, c.y as u8, c.z as u8]
}

pub fn color_to_rgba8(ctx: Struct) -> Vec<u8> {
    let c: Vec4 = ctx.get(&COLOR.into()).into();
    let c = c * 255.0;
    vec![c.x as u8, c.y as u8, c.z as u8, c.w as u8]
}

pub fn rasterize<P>(
    module: Module,
    width: u32,
    height: u32,
    pixel: impl Send + Sync + Fn(Struct) -> Vec<P::Subpixel>,
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Debug + Pixel,
    P::Subpixel: Send + Sync,
{
    let shape = dispatch_module(module);

    let indices: Vec<_> = (0..height)
        .into_iter()
        .flat_map(move |y| (0..width).into_iter().map(move |x| (x, y)))
        .collect();

    let sample = |x, y| {
        let ctx = Struct::new(StructIdentifier(CONTEXT))
            .set(
                POSITION_2D.into(),
                Value::Struct(
                    Struct::new(StructIdentifier(VECTOR2))
                        .set(X.into(), (((x as f32 / width as f32) - 0.5) * 2.0).into())
                        .set(Y.into(), (((y as f32 / height as f32) - 0.5) * -2.0).into()),
                ),
            )
            .set(
                ASPECT.into(),
                Value::Number(Number::Float(width as f64 / height as f64)),
            );

        let ctx = shape(ctx);

        pixel(ctx)
    };

    let pixels = {
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

            let chunk_size = (width * height) as usize / num_cpus::get();

            indices
                .into_par_iter()
                .chunks(chunk_size)
                .flat_map(|indices| {
                    indices
                        .into_iter()
                        .flat_map(|(x, y)| sample(x, y))
                        .collect::<Vec<_>>()
                })
                .collect()
        }

        #[cfg(not(feature = "rayon"))]
        {
            indices
                .into_iter()
                .flat_map(|(x, y)| sample(x, y))
                .collect()
        }
    };

    ImageBuffer::from_vec(width, height, pixels).expect("Failed to create image")
}
