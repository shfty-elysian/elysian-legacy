use std::{
    error::Error,
    io::{stdin, Read},
};

use elysian_shapes::shape::Shape;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::default();
    stdin().read_to_string(&mut buf)?;

    let result: Box<dyn Shape> = ron::from_str(&buf)?;
    println!("{result:#?}");

    Ok(())
}
