use elysian_ir::{module::{Module, Evaluate, EvaluateError}, ast::Struct};

use crate::Interpreter;

/// Interpreting evaluator
#[derive(Debug, Copy, Clone, Hash)]
pub struct Interpreted<'a>(pub &'a Module);

impl<'a> Evaluate<'a> for Interpreted<'a> {
    fn evaluate(&self, context: Struct) -> Result<Struct, EvaluateError> {
        let module = &self.0;

        /*
        println!(
            "Dispatching {} to interpreter",
            module.entry_point.name_unique()
        );
        */

        Ok(Interpreter {
            context,
            ..Default::default()
        }
        .evaluate(module))
    }
}

