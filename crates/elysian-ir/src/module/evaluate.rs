use std::{error::Error, fmt::Display};

use crate::ast::Struct;

pub type EvaluateError = Box<dyn Error + Send + Sync>;

pub trait Evaluate<'a>: 'a + Send + Sync {
    fn evaluate(&self, context: Struct) -> Result<Struct, EvaluateError>;
}

pub struct Dispatch<'a>(pub Vec<Box<dyn Evaluate<'a>>>);

impl<'a> Evaluate<'a> for Dispatch<'a> {
    fn evaluate(&self, context: Struct) -> Result<Struct, EvaluateError> {
        let mut errors = Vec::with_capacity(self.0.len());

        for evaluator in self.0.iter() {
            match evaluator.evaluate(context.clone()) {
                Ok(out) => return Ok(out),
                Err(e) => errors.push(e),
            }
        }

        Err(Box::new(DispatchError(errors)))
    }
}

#[derive(Debug, Default)]
struct DispatchError(Vec<EvaluateError>);

impl Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.0 {
            error.fmt(f)?;
        }

        Ok(())
    }
}

impl Error for DispatchError {}
