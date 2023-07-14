use std::fmt::Debug;

use image::{Rgb, RgbImage};
use rust_gpu_bridge::glam::Vec2;

use crate::{
    elysian::{expand::Expand, Elysian},
    frontend::{interpreter::evaluate_module, ToGlam},
    ir::{
        ast::{Struct, DISTANCE, GRADIENT, POSITION},
        from_elysian::elysian_module,
    },
};

use super::interpreter::Interpreter;

pub fn rasterize<N, V>(shape: &Elysian<N, V>, width: u32, height: u32, scale: f32) -> RgbImage
where
    N: Debug + Copy,
    V: Debug + Copy,
    Elysian<N, V>: ToGlam<2, Output = Elysian<f32, Vec2>>,
{
    let module = elysian_module(&shape.expand().to_glam());

    let mut image = RgbImage::new(width, height);

    for y in 0..width {
        for x in 0..height {
            let ctx = Struct::default().set_vector(
                POSITION,
                Vec2::new(
                    ((x as f32 / width as f32) - 0.5) * 2.0 / scale,
                    ((y as f32 / height as f32) - 0.5) * 2.0 / scale,
                ),
            );
            let ctx = evaluate_module(Interpreter { context: ctx }, &module).context;
            let d: f32 = ctx.get_number(&DISTANCE);
            let g: Vec2 = ctx.get_vector(&GRADIENT);

            let c = if d.abs() < 2.0 / width as f32 {
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
            };

            image.put_pixel(x, y, Rgb(c));
        }
    }

    image
}
