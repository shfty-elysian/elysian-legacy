pub mod core {
    pub use elysian_core::*;
}

pub mod macros {
    pub use elysian_proc_macros::*;
    pub use elysian_decl_macros::*;
}

#[cfg(feature = "syn")]
pub mod syn {
    pub use elysian_syn::*;
}

#[cfg(feature = "static")]
pub mod r#static {
    pub use elysian_static::*;
}

#[cfg(feature = "image")]
pub mod image {
    pub use elysian_image::*;
}

#[cfg(feature = "naga")]
pub mod naga {
    pub use elysian_naga::*;
}

#[cfg(feature = "shadertoy")]
pub mod shadertoy {
    pub use elysian_shadertoy::*;
}
