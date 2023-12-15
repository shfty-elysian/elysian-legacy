use rhai::{Array, Dynamic, EvalAltResult, FnPtr, NativeCallContext};

use crate::{Context, Evaluate};

#[derive(Debug, Clone, Hash)]
pub struct FoldFunction(FnPtr);

#[derive(Debug, Clone, Hash)]
pub struct Fold(Array, FoldFunction);

/// Given a list of evaluables, a list of domains, and an initial context,
/// fold-evaluate into a final context.
impl Fold {
    pub fn new(array: Array, combine: FnPtr) -> Self {
        Fold(array, FoldFunction(combine))
    }
}

impl Evaluate for Fold {
    fn evaluate(
        context: NativeCallContext,
        Fold(mut array, combine): Self,
        ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        let first = array.remove(0);
        let first = context.call_fn::<Context>("evaluate", (first, ctx.clone()))?;

        Ok(Dynamic::from(array.into_iter().try_fold(
            first,
            |acc, next| {
                let next = context.call_fn::<Context>("evaluate", (next, ctx.clone()))?;
                combine.0.call_within_context(&context, (acc, next))
            },
        )?))
    }
}
