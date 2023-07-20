use elysian::{
    core::ir::module::SpecializationData,
    syn::{elysian_to_syn, prettyplease},
};

fn main() {
    let source = elysian_to_syn(
        &test_shapes::point(),
        &SpecializationData::new_2d(),
        "test",
    );
    let source = prettyplease::unparse(&source);
    println!("{source:}");
}
