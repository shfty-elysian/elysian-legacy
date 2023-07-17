use elysian_syn::static_shapes;

fn main() {
    static_shapes(test_shapes::shapes());

    println!("cargo:rerun-if-changed=build.rs");
}
