use std::error::Error;

use elysian::{core::ir::module::SpecializationData, naga::module_to_naga};
use naga::{
    back::wgsl::WriterFlags,
    valid::{Capabilities, ValidationFlags},
};

fn main() {
    handle_result(main_impl())
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    let module = module_to_naga(&test_shapes::kettle_bell(), &SpecializationData::new_2d(), "test");
    println!("{module:#?}\n");

    let mut validator =
        naga::valid::Validator::new(ValidationFlags::all(), Capabilities::default());
    let module_info = validator.validate(&module)?;
    println!("{module_info:#?}\n");

    let out = naga::back::wgsl::write_string(&module, &module_info, WriterFlags::EXPLICIT_TYPES)?;
    println!("Output:\n{out:}");

    Ok(())
}

fn handle_result<T>(result: Result<T, Box<dyn Error>>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            handle_error(e.as_ref());
            eprintln!();
            panic!()
        }
    }
}

fn handle_error(e: &dyn Error) {
    eprintln!("{e:}");
    if let Some(source) = e.source() {
        handle_error(source);
    }
}
