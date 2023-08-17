use std::error::Error;

use elysian_ir::{module::{Module, Evaluate}, ast::Struct};

use crate::Interpreter;

/// Interpreting evaluator
#[derive(Debug, Clone, Hash)]
pub struct Interpreted<'a>(pub &'a Module);

impl<'a> Evaluate<'a> for Interpreted<'a> {
    fn evaluate(&self, context: Struct) -> Result<Struct, Box<dyn Error + Send + Sync>> {
        let module = &self.0;

        println!(
            "Dispatching {} to interpreter",
            module.entry_point.name_unique()
        );

        Ok(Interpreter {
            context,
            ..Default::default()
        }
        .evaluate(module))
    }
}

