use std::any::TypeId;

use crate::{
    Buffer, Channels, Context, Evaluate, Fold, LiftContextInput, Sample, Sequence, Vector,
};
use glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4};
use image::DynamicImage;
use rhai::{
    def_package,
    packages::{BasicArrayPackage, BasicMapPackage, BasicMathPackage, CorePackage},
    Array, Dynamic, EvalAltResult, FnPtr, FLOAT, RegisterNativeFunction,
};
use viuer::Config;

def_package! {
    pub ElysianPackage(module) : CorePackage, BasicMathPackage, BasicArrayPackage, BasicMapPackage
    {
    } |> |engine| {
        FLOAT::register(engine);

        match TypeId::of::<FLOAT>() {
            t if t == TypeId::of::<f32>() => {
                Vec2::register(engine);
                Vec3::register(engine);
                Vec4::register(engine);
            }
            t if t == TypeId::of::<f64>() => {
                DVec2::register(engine);
                DVec3::register(engine);
                DVec4::register(engine);
            }
            _ => panic!("Unrecognized float type {}", std::any::type_name::<FLOAT>())
        }

        DynamicImage::register(engine);
        engine
            .register_fn("image_luma8", |x: i64, y: i64| DynamicImage::new_luma8(x as u32, y as u32))
            .register_fn("image_luma_a8", |x: i64, y: i64| DynamicImage::new_luma_a8(x as u32, y as u32))
            .register_fn("image_rgb8", |x: i64, y: i64| DynamicImage::new_rgb8(x as u32, y as u32))
            .register_fn("image_rgba8", |x: i64, y: i64| DynamicImage::new_rgba8(x as u32, y as u32))
            .register_fn("image_luma16", |x: i64, y: i64| DynamicImage::new_luma16(x as u32, y as u32))
            .register_fn("image_luma_a16", |x: i64, y: i64| DynamicImage::new_luma_a16(x as u32, y as u32))
            .register_fn("image_rgb16", |x: i64, y: i64| DynamicImage::new_rgb16(x as u32, y as u32))
            .register_fn("image_rgba16", |x: i64, y: i64| DynamicImage::new_rgba16(x as u32, y as u32))
            .register_fn("image_rgb32f", |x: i64, y: i64| DynamicImage::new_rgb32f(x as u32, y as u32))
            .register_fn("image_rgba32f", |x: i64, y: i64| DynamicImage::new_rgba32f(x as u32, y as u32))
            ;

        engine.register_fn("viuer", |image: &mut DynamicImage| {
            viuer::print(image, &Config {
                transparent: false,
                absolute_offset: false,
                x: 0, y: 0,
                restore_cursor: false,
                width: None,
                height: None,
                truecolor: true,
                use_kitty: true,
                use_iterm: false
            }).unwrap();
        });

        FnPtr::register_evaluate(engine);

        engine.register_type_with_name::<LiftContextInput>("lift_context_input")
            .register_fn("lift_context_input", LiftContextInput::new);
        LiftContextInput::register_evaluate(engine);
        LiftContextInput::register_sample(engine);

        engine.register_type_with_name::<Channels>("channels")
            .register_fn("channels", Channels::new);
        Channels::register_evaluate(engine);
        Channels::register_sample(engine);

        engine.register_type_with_name::<Sequence>("sequence")
            .register_fn("sequence", <Sequence as From<Array>>::from)
            .register_fn("sequence", <Sequence as From<Channels>>::from)
            .register_fn("sample", Sequence::sample);
        Sequence::register_evaluate(engine);
        Sequence::register_sample(engine);

        engine.register_type_with_name::<Fold>("fold")
            .register_fn("fold", Fold::new)
            .register_fn("evaluate", Fold::evaluate)
            .register_fn("sample", Fold::sample);
        Fold::register_evaluate(engine);
        Fold::register_sample(engine);

        engine.register_type_with_name::<Context>("context")
            .register_indexer_get(|context: &mut Context, key: &str| -> Result<Dynamic, Box<EvalAltResult>> {
                Ok(context.get(key).cloned()?)
            })
            .register_indexer_set(Context::set)
            .register_fn("context", Context::new)
            .register_fn("contains", Context::contains)
            .register_fn("+", Context::concat)
            .register_fn("+=", Context::append);
    }
}
