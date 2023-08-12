use std::error::Error;

use elysian::{
    ir::module::{IntoAsIR, SpecializationData},
    shadertoy::module_to_shadertoy,
};

fn main() {
    env_logger::init();
    handle_result(main_impl())
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    println!(
        "{:}",
        module_to_shadertoy(
            &test_shapes::test_shape()
                .as_ir()
                .module(&SpecializationData::new_2d())
        )?
    );
    Ok(())
}

fn handle_result<T>(result: Result<T, Box<dyn Error>>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            handle_error(e.as_ref());
            eprintln!();
            panic!("{e:#?}")
        }
    }
}

fn handle_error(e: &dyn Error) {
    log::error!("{e:}");
    if let Some(source) = e.source() {
        handle_error(source);
    }
}
