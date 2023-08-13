use std::{
    error::Error,
    io::{stdin, stdout, BufWriter, Cursor, Read, Write},
};

use elysian_image::{distance_to_luma_8, rasterize};
use elysian_ir::module::SpecializationData;
use elysian_shapes::shape::Shape;
use image::{ImageOutputFormat, Luma};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::default();
    stdin().read_to_string(&mut buf)?;

    let shape: Box<dyn Shape> = ron::from_str(&buf)?;
    let module = shape.module(&SpecializationData::new_2d());

    let (width, height) = (8, 4);
    let image = rasterize::<Luma<u8>>(module, width, height, distance_to_luma_8);

    let mut buf = vec![];
    let mut writer = BufWriter::new(Cursor::new(&mut buf));
    image.write_to(&mut writer, ImageOutputFormat::Png)?;
    drop(writer);

    stdout().lock().write(&buf)?;

    Ok(())
}
