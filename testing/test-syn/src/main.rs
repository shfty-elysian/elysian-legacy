use elysian::{
    ir::module::SpecializationData,
    syn::{module_to_syn, prettyplease},
};

fn main() {
    let source = module_to_syn(
        &test_shapes::test_shape(),
        &SpecializationData::new_2d(),
        "test",
    );
    let source = prettyplease::unparse(&source);
    println!("{source:}");
}
