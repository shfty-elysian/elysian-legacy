use elysian::{
    ascii::{ascii, ASCII_RAMP},
    ir::module::SpecializationData,
    r#static::include_static_shapes,
};

include_static_shapes!();

fn main() {
    let shape = test_shapes::test_shape().module(&SpecializationData::new_2d());

    let (width, height) = (64, 48);

    let ascii = ascii(shape, width, height, ASCII_RAMP, 0.4);
    println!("{ascii:}");
}
