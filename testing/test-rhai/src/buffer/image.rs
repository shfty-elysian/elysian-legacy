use glam::DVec2;
use image::{DynamicImage, ImageBuffer};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rhai::{Array, Dynamic, EvalAltResult, NativeCallContext, Scope, Shared, FLOAT};

use crate::{engine, Buffer, Vector};

impl Buffer for DynamicImage {
    fn map<'a>(
        context: NativeCallContext,
        this: &'a mut Self,
        shape: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        let width = this.width();
        let height = this.height();

        let w64 = width as f64;
        let h64 = height as f64;
        let aspect = w64.max(h64) / w64.min(h64);
        let aspect = if w64 < h64 {
            DVec2::new(1.0, aspect)
        } else {
            DVec2::new(aspect, 1.0)
        };

        let buf = (0..height)
            .into_par_iter()
            .flat_map(|y| (0..width).into_par_iter().map(move |x| (x, y)))
            .map(|(x, y)| {
                let pos = ((DVec2::new(x as f64, y as f64)
                    / DVec2::new(width as f64, height as f64))
                    * 2.0
                    - 1.0)
                    * aspect;

                let mut engine = engine();

                // Load existing imports into new engine instance
                for (name, module) in context.global_runtime_state().iter_imports() {
                    engine.register_static_module(name, Shared::new(module.clone()));
                    for (name, module) in module.iter_sub_modules() {
                        engine.register_static_module(name, Shared::new((**module).clone()));
                    }
                }

                DVec2::register_new(&mut engine, "vec");

                let mut scope = Scope::new();
                scope.push_constant("shape", shape.clone());
                scope.push_constant("position", pos);

                Ok(engine
                    .eval_with_scope::<Array>(&mut scope, "sample(shape, position)")?
                    .into_iter()
                    .map(|t: Dynamic| t.as_float())
                    .collect::<Result<Vec<_>, _>>()?)
            })
            .collect::<Result<Vec<_>, Box<EvalAltResult>>>()?
            .into_iter()
            .flatten();

        let error = || {
            EvalAltResult::ErrorRuntime("Failed to create ImageBuffer".into(), context.position())
        };

        let as_u8 = |t: FLOAT| (t * u8::MAX as FLOAT).round() as u8;
        let as_u16 = |t: FLOAT| (t * u16::MAX as FLOAT).round() as u16;
        let as_f32 = |t: FLOAT| t as f32;

        macro_rules! impl_image {
            (u8, $image:expr) => {
                *$image =
                    ImageBuffer::from_vec($image.width(), $image.height(), buf.map(as_u8).collect())
                        .ok_or_else(error)?
            };
            (u16, $image:expr) => {
                *$image = ImageBuffer::from_vec(
                    $image.width(),
                    $image.height(),
                    buf.map(as_u16).collect(),
                )
                .ok_or_else(error)?
            };
            (f32, $image:expr) => {
                *$image = ImageBuffer::from_vec(
                    $image.width(),
                    $image.height(),
                    buf.map(as_f32).collect(),
                )
                .ok_or_else(error)?
            };
        }

        match this {
            DynamicImage::ImageLuma8(image) => impl_image!(u8, image),
            DynamicImage::ImageLumaA8(image) => impl_image!(u8, image),
            DynamicImage::ImageRgb8(image) => impl_image!(u8, image),
            DynamicImage::ImageRgba8(image) => impl_image!(u8, image),
            DynamicImage::ImageLuma16(image) => impl_image!(u16, image),
            DynamicImage::ImageLumaA16(image) => impl_image!(u16, image),
            DynamicImage::ImageRgb16(image) => impl_image!(u16, image),
            DynamicImage::ImageRgba16(image) => impl_image!(u16, image),
            DynamicImage::ImageRgb32F(image) => impl_image!(f32, image),
            DynamicImage::ImageRgba32F(image) => impl_image!(f32, image),
            _ => unimplemented!(),
        }

        Ok(())
    }
}
