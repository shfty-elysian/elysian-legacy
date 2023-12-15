use rhai::{module_resolvers::FileModuleResolver, packages::Package, Engine, Position};

use crate::{ElysianPackage, GlobalModuleResolver};

pub fn engine() -> Engine {
    let mut engine = Engine::new_raw();

    engine.on_print(|text| {
        println!("{text}");
    });

    engine.on_debug(|text, source, pos| match (source, pos) {
        (Some(source), Position::NONE) => println!("{source} | {text}"),
        (Some(source), pos) => println!("{source} @ {pos:?} | {text}"),
        (None, Position::NONE) => println!("{text}"),
        (None, pos) => println!("{pos:?} | {text}"),
    });

    engine.set_module_resolver(GlobalModuleResolver::new(FileModuleResolver::new()));

    ElysianPackage::new().register_into_engine(&mut engine);

    engine
}
