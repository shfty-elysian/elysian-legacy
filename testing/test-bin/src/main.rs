use std::time::Instant;

use elysian::{
    core::ir::module::SpecializationData, image::rasterize, r#static::include_static_shapes,
};
use viuer::Config;

include_static_shapes!();

fn main() {
    let shape = test_shapes::test_shape();

    let start = Instant::now();
    let image = rasterize(&shape, &SpecializationData::new_2d(), 64, 48, 0.5);
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
