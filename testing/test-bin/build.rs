use elysian_core::ir::module::SpecializationData;
use elysian_syn::static_shapes::static_shapes;

fn main() {
    static_shapes(test_shapes::shapes(), &SpecializationData::new_2d());

    println!("cargo:rerun-if-changed=build.rs");
}
