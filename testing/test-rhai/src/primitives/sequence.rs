use rhai::{Array, Dynamic, EvalAltResult, NativeCallContext};

use crate::{Channels, Context, Evaluate};

/// A sequence of field functions
#[derive(Debug, Default, Clone, Hash)]
pub struct Sequence(Array);

impl From<Channels> for Sequence {
    fn from(value: Channels) -> Self {
        Sequence(vec![Dynamic::from(value)])
    }
}

impl From<Array> for Sequence {
    fn from(value: Array) -> Self {
        Sequence(value)
    }
}

impl Evaluate for Sequence {
    /// Given a list of commands, list of filter keys, and a context,
    /// evaluate each command in sequence and return the updated context.
    fn evaluate(
        context: NativeCallContext,
        this: Self,
        ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        this.0
            .into_iter()
            .try_fold(Dynamic::from(ctx), |acc, next| {
                context.call_fn("evaluate", (next, acc))
            })
    }
}
