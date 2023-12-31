//! Precompile shapes into Rust functions via build.rs

mod evaluator;

pub use evaluator::*;

use elysian_ir::module::Module;
use elysian_syn::module_to_string;

use std::{collections::BTreeMap, sync::OnceLock};

use elysian_ir::ast::Struct;

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
pub fn static_shapes<'a, T: IntoIterator<Item = (&'a str, Module)>>(t: T) {
    let source: String = t
        .into_iter()
        .map(|(name, module)| module_to_string(&module, name))
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
