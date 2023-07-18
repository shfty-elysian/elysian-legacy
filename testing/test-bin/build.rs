use elysian_syn::static_shapes::static_shapes_f32;

fn main() {
    static_shapes_f32(test_shapes::shapes());

    println!("cargo:rerun-if-changed=build.rs");
}
