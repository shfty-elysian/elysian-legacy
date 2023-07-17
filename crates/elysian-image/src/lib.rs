use image::RgbImage;
use rust_gpu_bridge::glam::Vec2;
use tracing::instrument;

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use elysian_core::{
    ast::Elysian,
    ir::ast::{Struct, DISTANCE, GRADIENT, POSITION},
};
use elysian_syn::dispatch_shape;

#[instrument]
pub fn rasterize(shape: &Elysian<f32, Vec2>, width: u32, height: u32, scale: f32) -> RgbImage {
    let shape = dispatch_shape(shape);

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
                    let ctx = Struct::default().set_vector(
                        POSITION,
                        Vec2::new(
                            ((x as f32 / width as f32) - 0.5) * 2.0 / scale,
                            ((y as f32 / height as f32) - 0.5) * 2.0 / scale,
                        ),
                    );

                    let ctx = shape(ctx);

                    let d: f32 = ctx.get_number(&DISTANCE);
                    let g: Vec2 = ctx.get_vector(&GRADIENT);

                    if d.abs() < 2.0 / width as f32 {
                        [255, 255, 255]
                    } else if d < 0.0 {
                        [
                            ((g.x * 0.5 + 0.5) * 255.0).round() as u8,
                            ((g.y * 0.5 + 0.5) * 255.0).round() as u8,
                            255,
                        ]
                    } else {
                        [
                            ((g.x * 0.5 + 0.5) * 255.0).round() as u8,
                            ((g.y * 0.5 + 0.5) * 255.0).round() as u8,
                            0,
                        ]
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    RgbImage::from_vec(width, height, pixels).expect("Failed to create image")
}
