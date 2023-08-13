use elysian::{
    syn::{module_to_syn, prettyplease},
};

fn main() {
    let source = module_to_syn(
        &test_shapes::test_shape(),
        "test",
    );
    let source = prettyplease::unparse(&source);
    println!("{source:}");
}
