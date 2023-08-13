use ron::ser::PrettyConfig;

fn main() {
    let shape = test_shapes::test_shape();
    let result = ron::ser::to_string_pretty(&shape, PrettyConfig::new()).unwrap();
    println!("{result:}");
}
