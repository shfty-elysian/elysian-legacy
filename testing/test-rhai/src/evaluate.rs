use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, NativeCallContext};

use crate::Context;

/// Evaluate a primitive with the given context
pub trait Evaluate: 'static + Clone + Send + Sync {
    fn evaluate(
        context: NativeCallContext,
        this: Self,
        ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>>;

    fn register_evaluate(engine: &mut Engine) {
        engine.register_fn("evaluate", Self::evaluate);
    }
}

impl Evaluate for FnPtr {
    fn evaluate(
        context: NativeCallContext,
        this: Self,
        ctx: Context,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        this.call_within_context(&context, (ctx,))
    }
}
