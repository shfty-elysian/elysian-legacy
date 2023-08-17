use std::{error::Error, fmt::Display};

use crate::ast::{Struct, Value, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z};

use super::{StructIdentifier, CONTEXT};

pub type EvaluateError = Box<dyn Error + Send + Sync>;

pub trait Evaluate<'a>: 'a + Send + Sync {
    fn evaluate(&self, context: Struct) -> Result<Struct, EvaluateError>;

    fn sample_2d(&self, p: [f64; 2]) -> Result<Struct, EvaluateError> {
        self.evaluate(
            Struct::new(CONTEXT.into()).set(
                POSITION_2D.into(),
                Value::Struct(
                    Struct::new(StructIdentifier(VECTOR2))
                        .set(X.into(), p[0].into())
                        .set(Y.into(), p[1].into()),
                ),
            ),
        )
    }

    fn sample_3d(&self, p: [f64; 3]) -> Result<Struct, EvaluateError> {
        self.evaluate(
            Struct::new(CONTEXT.into()).set(
                POSITION_3D.into(),
                Value::Struct(
                    Struct::new(StructIdentifier(VECTOR3))
                        .set(X.into(), p[0].into())
                        .set(Y.into(), p[1].into())
                        .set(Z.into(), p[2].into()),
                ),
            ),
        )
    }
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
