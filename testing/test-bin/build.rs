use elysian_core::ir::module::SpecializationData;
use elysian_static::static_shapes;

fn main() {
    static_shapes(test_shapes::shapes(), &SpecializationData::new_2d());

    println!("cargo:rerun-if-changed=build.rs");
}
