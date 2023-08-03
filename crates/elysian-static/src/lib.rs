use elysian_core::ir::module::SpecializationData;
use elysian_interpreter::{evaluate_module, Interpreter};
use elysian_syn::module_to_string;

use std::{collections::BTreeMap, sync::OnceLock};

use elysian_core::ir::{ast::Struct, module::AsModule};

pub type ShapeHash = u64;
pub type ShapeFn = fn(Struct) -> Struct;

pub struct StaticShape {
    pub hash: ShapeHash,
    pub function: ShapeFn,
}

impl Clone for StaticShape {
    fn clone(&self) -> Self {
        Self {
            hash: self.hash.clone(),
            function: self.function.clone(),
        }
    }
}

impl Copy for StaticShape {}

/// Distributed slice of shape hash -> shape function pairs
/// Populated at link-time by auto-generated shape modules
#[linkme::distributed_slice]
pub static STATIC_SHAPES: [StaticShape] = [..];

/// Runtime storage for static shape data
static STATIC_SHAPES_MAP: OnceLock<BTreeMap<ShapeHash, ShapeFn>> = OnceLock::new();

/// Accessor for STATIC_SHAPES_MAP_F32
pub fn static_shapes_map() -> &'static BTreeMap<ShapeHash, ShapeFn> {
    STATIC_SHAPES_MAP.get_or_init(|| {
        STATIC_SHAPES
            .into_iter()
            .copied()
            .map(|t| (t.hash, t.function))
            .collect()
    })
    //STATIC_SHAPES_MAP.get_or_init(Default::default)
}

/// Build.rs static shape registrar
pub fn static_shapes<'a, T: IntoIterator<Item = (&'a str, Box<dyn AsModule>)>>(
    t: T,
    spec: &SpecializationData,
) {
    let source: String = t
        .into_iter()
        .map(|(name, shape)| module_to_string(&shape, spec, name))
        .collect();

    let out_dir = std::env::var_os("OUT_DIR").expect("No OUT_DIR environment variable");
    let dest_path = std::path::Path::new(&out_dir).join("static_shapes.rs");
    std::fs::write(&dest_path, source).unwrap();
}

/// Convenience macro for including generated static shape code
#[macro_export]
macro_rules! include_static_shapes {
    () => {
        include!(concat!(env!("OUT_DIR"), "/static_shapes.rs"));
    };
}

/// Return a function that calls the static implementation of a given shape if it exists,
/// falling back to the interpreter otherwise.
pub fn dispatch_shape<T>(
    shape: &T,
    spec: &SpecializationData,
) -> Box<dyn Fn(Struct) -> Struct + Send + Sync>
where
    T: AsModule,
{
    let hash = shape.hash_ir();

    if let Some(f) = static_shapes_map().get(&hash) {
        println!("Dispatching to static function");
        Box::new(|context| f(context))
    } else {
        println!("Dispatching to dynamic interpreter");
        let module = shape.module(spec);
        Box::new(move |context| {
            evaluate_module(
                Interpreter {
                    context,
                    ..Default::default()
                },
                &module,
            )
        })
    }
}
