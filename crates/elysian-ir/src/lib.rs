extern crate self as elysian_ir;
pub mod ast;
pub mod module;

#[cfg(feature = "quote")]
mod to_tokens {
    #[cfg(feature = "internal")]
    #[macro_export]
    macro_rules ! quote_crate {
        ($($p:tt)*) => {
            quote!(elysian_core::$($p)*)
        }
    }

    #[cfg(not(feature = "internal"))]
    #[macro_export]
    macro_rules ! quote_crate {
        ($($p:tt)*) => {
            quote!(elysian::core::$($p)*)
        }
    }
}
