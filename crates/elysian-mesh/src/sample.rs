use elysian_ir::{
    ast::{Struct, Value, POSITION_2D, POSITION_3D, VECTOR2, VECTOR3, X, Y, Z},
    module::{Evaluate, EvaluateError, StructIdentifier, CONTEXT},
};

use crate::vector_space::{DimensionVector, VectorSpace, D2, D3};

/// Given a position, sample it and return the result
pub trait Sample<'a, D: VectorSpace<f64>>: Evaluate<'a> {
    fn sample(&self, p: D::DimensionVector) -> Result<Struct, EvaluateError>;
}

impl<'a, T> Sample<'a, D2> for T
where
    T: Evaluate<'a>,
{
    fn sample(
        &self,
        p: <D2 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<Struct, EvaluateError> {
        self.evaluate(
            Struct::new(CONTEXT.into()).set(
                POSITION_2D.into(),
                Value::Struct(
                    Struct::new(StructIdentifier(VECTOR2))
                        .set(X.into(), p.x.into())
                        .set(Y.into(), p.y.into()),
                ),
            ),
        )
    }
}

impl<'a, T> Sample<'a, D3> for T
where
    T: Evaluate<'a>,
{
    fn sample(
        &self,
        p: <D3 as DimensionVector<f64>>::DimensionVector,
    ) -> Result<Struct, EvaluateError> {
        self.evaluate(
            Struct::new(CONTEXT.into()).set(
                POSITION_3D.into(),
                Value::Struct(
                    Struct::new(StructIdentifier(VECTOR3))
                        .set(X.into(), p.x.into())
                        .set(Y.into(), p.y.into())
                        .set(Z.into(), p.z.into()),
                ),
            ),
        )
    }
}
