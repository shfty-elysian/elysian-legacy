use std::{error::Error, fmt::Display};

use elysian_ir::{
    ast::Struct,
    module::{Evaluate, Module, EvaluateError},
};

use crate::static_shapes_map;

// Evaluator for precompiled functions
#[derive(Debug, Copy, Clone, Hash)]
pub struct Precompiled<'a>(pub &'a Module);

impl<'a> Evaluate<'a> for Precompiled<'a> {
    fn evaluate(&self, context: Struct) -> Result<Struct, EvaluateError> {
        let module = &self.0;

        let Some(f) = static_shapes_map().get(&module.hash) else {
        return Err(Box::new(PrecompiledError::MissingFunction(module.entry_point.name_unique())))
    };

        /*
        println!(
            "Dispatching {} to precompiled function",
            module.entry_point.name_unique()
        );
        */

        Ok(f(context))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrecompiledError {
    MissingFunction(String),
}

impl Display for PrecompiledError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrecompiledError::MissingFunction(name) => {
                f.write_str(&format!("Missing precompiled shape for {}", name))
            }
        }
    }
}

impl Error for PrecompiledError {}
