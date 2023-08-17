use std::error::Error;

use elysian_image::{distance_to_luma_32, rasterize};
use elysian_interpreter::Interpreted;
use elysian_ir::module::{Dispatch, Module};
use elysian_static::Precompiled;
use image::Luma;

pub const ASCII_RAMP: &'static str = " .:-=+*#%@";

pub fn ascii(
    module: Module,
    width: u32,
    height: u32,
    ramp: &str,
    fac: f32,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let module = module;
    let buf = rasterize::<Luma<f32>>(
        Dispatch(vec![
            Box::new(Precompiled(&module)),
            Box::new(Interpreted(&module)),
        ]),
        width,
        height,
        distance_to_luma_32,
    )?;
    let ramp = ramp.chars().collect::<Vec<_>>();

    let rows = buf.rows().collect::<Vec<_>>();
    let rows = rows
        .chunks(2)
        .map(|chunk| {
            chunk[0]
                .clone()
                .into_iter()
                .zip(chunk[1].clone().into_iter())
                .map(|(a, b)| (a[0] + b[0]) * 0.5)
        })
        .collect::<Vec<_>>();

    Ok(rows
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .map(|val| {
                    let idx =
                        ((fac * val / (ramp.len() - 1) as f32) as usize).clamp(0, ramp.len() - 1);
                    ramp[idx]
                })
                .chain(['\n'])
        })
        .collect())
}
